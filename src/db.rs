use crate::config::Config;
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::Uuid;

/// Task priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Low,
            2 => Self::Medium,
            3 => Self::High,
            _ => Self::None,
        }
    }

    /// User-facing label.
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }

    /// Emoji for display in task rows.
    pub fn icon(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Low => "🔵",
            Self::Medium => "🟡",
            Self::High => "🔴",
        }
    }

    /// CSS class name suffix.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Low => "priority-low",
            Self::Medium => "priority-medium",
            Self::High => "priority-high",
        }
    }
}

/// A named collection of tasks.
#[derive(Debug, Clone)]
pub struct TaskList {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub position: i32,
    pub created_at: String,
}

/// A single task belonging to a list.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub list_id: String,
    pub title: String,
    pub notes: String,
    pub completed: bool,
    pub priority: Priority,
    pub due_date: Option<String>,
    pub position: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    /// Formatted due-date string for display (e.g. "Jul 18" or "Overdue").
    pub fn due_display(&self) -> Option<String> {
        let raw = self.due_date.as_deref()?;
        if raw.is_empty() {
            return None;
        }
        // Parse YYYY-MM-DD
        let parsed = chrono::NaiveDate::parse_from_str(raw, "%Y-%m-%d").ok()?;
        let today = chrono::Local::now().date_naive();
        let diff = (parsed - today).num_days();

        Some(match diff {
            d if d < 0 => format!("Overdue ({})", parsed.format("%b %-d")),
            0 => "Today".to_string(),
            1 => "Tomorrow".to_string(),
            _ => parsed.format("%b %-d").to_string(),
        })
    }

    /// Whether the task is overdue.
    pub fn is_overdue(&self) -> bool {
        self.due_date
            .as_deref()
            .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .map_or(false, |parsed| {
                parsed < chrono::Local::now().date_naive() && !self.completed
            })
    }
}

/// Wrapper around a SQLite connection providing task-management operations.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open (or create) the database at `path`, run schema migrations,
    /// and seed default lists on first use.
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        // Performance and safety pragmas
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;
             PRAGMA synchronous = NORMAL;",
        )?;

        let db = Self { conn };
        db.create_schema()?;
        db.migrate()?;
        Ok(db)
    }

    /// Open a new connection to the same database.
    pub fn open_again(&self) -> Result<Self> {
        let path = Config::database_path();
        Self::open(&path)
    }

    /// Create tables and indexes if they don't already exist,
    /// then seed three default lists when the database is brand new.
    fn create_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS lists (
                id         TEXT    PRIMARY KEY NOT NULL,
                name       TEXT    NOT NULL,
                icon       TEXT    NOT NULL DEFAULT '📋',
                position   INTEGER NOT NULL DEFAULT 0,
                created_at TEXT    NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id         TEXT    PRIMARY KEY NOT NULL,
                list_id    TEXT    NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
                title      TEXT    NOT NULL,
                notes      TEXT    NOT NULL DEFAULT '',
                completed  INTEGER NOT NULL DEFAULT 0,
                priority   INTEGER NOT NULL DEFAULT 0,
                due_date   TEXT,
                position   INTEGER NOT NULL DEFAULT 0,
                created_at TEXT    NOT NULL,
                updated_at TEXT    NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_tasks_list
                ON tasks(list_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_list_completed
                ON tasks(list_id, completed);",
        )?;

        // Seed default lists on first run
        let count: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM lists", [], |r| r.get(0))?;

        if count == 0 {
            let now = Utc::now().to_rfc3339();
            let defaults = [("Inbox", "📥"), ("Personal", "🏠"), ("Work", "💼")];
            for (i, (name, icon)) in defaults.iter().enumerate() {
                self.conn.execute(
                    "INSERT INTO lists (id, name, icon, position, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![Uuid::new_v4().to_string(), name, icon, i as i32, &now],
                )?;
            }
            log::info!("Seeded default lists: Inbox, Personal, Work");
        }

        Ok(())
    }

    /// Run forward-only migrations for columns added after Phase 1.
    /// Each migration checks whether the column already exists before altering.
    fn migrate(&self) -> Result<()> {
        // Phase 2 columns — safe to run multiple times
        let columns = self.get_column_names("tasks")?;

        if !columns.contains(&"notes".to_string()) {
            self.conn.execute_batch(
                "ALTER TABLE tasks ADD COLUMN notes TEXT NOT NULL DEFAULT '';",
            )?;
            log::info!("Migration: added 'notes' column to tasks");
        }
        if !columns.contains(&"priority".to_string()) {
            self.conn.execute_batch(
                "ALTER TABLE tasks ADD COLUMN priority INTEGER NOT NULL DEFAULT 0;",
            )?;
            log::info!("Migration: added 'priority' column to tasks");
        }
        if !columns.contains(&"due_date".to_string()) {
            self.conn.execute_batch(
                "ALTER TABLE tasks ADD COLUMN due_date TEXT;",
            )?;
            log::info!("Migration: added 'due_date' column to tasks");
        }

        Ok(())
    }

    /// Helper: list column names for a given table.
    fn get_column_names(&self, table: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(&format!("PRAGMA table_info({})", table))?;
        let names = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(names)
    }

    // ─── Lists ───────────────────────────────────────────────────────────

    /// Return all lists ordered by position.
    pub fn get_lists(&self) -> Result<Vec<TaskList>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, icon, position, created_at
             FROM lists ORDER BY position",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(TaskList {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    icon: row.get(2)?,
                    position: row.get(3)?,
                    created_at: row.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Create a new list and return it.
    pub fn create_list(&self, name: &str, icon: &str) -> Result<TaskList> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let pos: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM lists",
            [],
            |r| r.get(0),
        )?;
        self.conn.execute(
            "INSERT INTO lists (id, name, icon, position, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&id, name, icon, pos, &now],
        )?;
        log::info!("Created list '{}' ({})", name, id);
        Ok(TaskList {
            id,
            name: name.into(),
            icon: icon.into(),
            position: pos,
            created_at: now,
        })
    }

    /// Delete a list and all its tasks (CASCADE).
    pub fn delete_list(&self, list_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM lists WHERE id = ?1", params![list_id])?;
        log::info!("Deleted list {}", list_id);
        Ok(())
    }

    // ─── Tasks ───────────────────────────────────────────────────────────

    /// Return tasks for a given list, filtered by completion status,
    /// ordered by position.
    pub fn get_tasks(&self, list_id: &str, completed: bool) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, list_id, title, notes, completed, priority, due_date,
                    position, created_at, updated_at
             FROM tasks
             WHERE list_id = ?1 AND completed = ?2
             ORDER BY position ASC",
        )?;
        let rows = stmt
            .query_map(params![list_id, completed as i32], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    list_id: row.get(1)?,
                    title: row.get(2)?,
                    notes: row.get(3)?,
                    completed: row.get::<_, i32>(4)? != 0,
                    priority: Priority::from_i32(row.get(5)?),
                    due_date: row.get(6)?,
                    position: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Fetch a single task by ID.
    pub fn get_task(&self, task_id: &str) -> Result<Task> {
        let task = self.conn.query_row(
            "SELECT id, list_id, title, notes, completed, priority, due_date,
                    position, created_at, updated_at
             FROM tasks WHERE id = ?1",
            params![task_id],
            |row| {
                Ok(Task {
                    id: row.get(0)?,
                    list_id: row.get(1)?,
                    title: row.get(2)?,
                    notes: row.get(3)?,
                    completed: row.get::<_, i32>(4)? != 0,
                    priority: Priority::from_i32(row.get(5)?),
                    due_date: row.get(6)?,
                    position: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            },
        )?;
        Ok(task)
    }

    /// Create a new task in the given list and return it.
    pub fn create_task(&self, list_id: &str, title: &str) -> Result<Task> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let pos: i32 = self.conn.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1
             FROM tasks WHERE list_id = ?1",
            params![list_id],
            |r| r.get(0),
        )?;
        self.conn.execute(
            "INSERT INTO tasks (id, list_id, title, notes, completed, priority, due_date, position, created_at, updated_at)
             VALUES (?1, ?2, ?3, '', 0, 0, NULL, ?4, ?5, ?6)",
            params![&id, list_id, title, pos, &now, &now],
        )?;
        log::info!("Created task '{}' in list {}", title, list_id);
        Ok(Task {
            id,
            list_id: list_id.into(),
            title: title.into(),
            notes: String::new(),
            completed: false,
            priority: Priority::None,
            due_date: None,
            position: pos,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Update a task's editable fields.
    pub fn update_task(
        &self,
        task_id: &str,
        title: &str,
        notes: &str,
        priority: Priority,
        due_date: Option<&str>,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE tasks
             SET title = ?1, notes = ?2, priority = ?3, due_date = ?4, updated_at = ?5
             WHERE id = ?6",
            params![title, notes, priority as i32, due_date, &now, task_id],
        )?;
        log::info!("Updated task {}", task_id);
        Ok(())
    }

    /// Toggle a task's completed flag. Returns the new value.
    pub fn toggle_task(&self, task_id: &str) -> Result<bool> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE tasks SET completed = NOT completed, updated_at = ?1
             WHERE id = ?2",
            params![&now, task_id],
        )?;
        let completed: bool = self.conn.query_row(
            "SELECT completed FROM tasks WHERE id = ?1",
            params![task_id],
            |r| r.get::<_, i32>(0).map(|v| v != 0),
        )?;
        log::debug!("Toggled task {} → completed={}", task_id, completed);
        Ok(completed)
    }

    /// Permanently delete a task.
    pub fn delete_task(&self, task_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id])?;
        log::info!("Deleted task {}", task_id);
        Ok(())
    }

    /// Count incomplete tasks in a list (used for tab badges).
    pub fn task_count(&self, list_id: &str) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks
             WHERE list_id = ?1 AND completed = 0",
            params![list_id],
            |r| r.get(0),
        )?;
        Ok(count)
    }
}
