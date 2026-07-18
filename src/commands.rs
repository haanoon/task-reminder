//! Command pattern implementation for Undo/Redo history.

use crate::db::Database;
use std::rc::Rc;

pub trait Command {
    fn execute(&mut self, db: &Database) -> Result<(), String>;
    fn undo(&mut self, db: &Database) -> Result<(), String>;
}

/// Command to create a task.
pub struct CreateTaskCommand {
    list_id: String,
    title: String,
    task_id: Option<String>,
}

impl CreateTaskCommand {
    pub fn new(list_id: &str, title: &str) -> Self {
        Self {
            list_id: list_id.to_string(),
            title: title.to_string(),
            task_id: None,
        }
    }
}

impl Command for CreateTaskCommand {
    fn execute(&mut self, db: &Database) -> Result<(), String> {
        match db.create_task(&self.list_id, &self.title) {
            Ok(task) => {
                self.task_id = Some(task.id);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn undo(&mut self, db: &Database) -> Result<(), String> {
        if let Some(ref id) = self.task_id {
            match db.delete_task(id) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("No task created to undo".to_string())
        }
    }
}

/// Command to delete a task.
pub struct DeleteTaskCommand {
    task_id: String,
    deleted_task: Option<crate::db::Task>,
}

impl DeleteTaskCommand {
    pub fn new(task_id: &str) -> Self {
        Self {
            task_id: task_id.to_string(),
            deleted_task: None,
        }
    }
}

impl Command for DeleteTaskCommand {
    fn execute(&mut self, db: &Database) -> Result<(), String> {
        match db.get_task(&self.task_id) {
            Ok(task) => {
                self.deleted_task = Some(task);
                match db.delete_task(&self.task_id) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn undo(&mut self, db: &Database) -> Result<(), String> {
        if let Some(ref task) = self.deleted_task {
            // Restore deleted task by creating it and writing fields
            match db.create_task(&task.list_id, &task.title) {
                Ok(new_task) => {
                    db.update_task(&new_task.id, &task.title, &task.notes, task.priority, task.due_date.as_deref()).ok();
                    if task.completed {
                        db.toggle_task(&new_task.id).ok();
                    }
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("No task deleted to restore".to_string())
        }
    }
}

/// History tracker for Command stacks.
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, mut cmd: Box<dyn Command>, db: &Database) -> Result<(), String> {
        cmd.execute(db)?;
        self.undo_stack.push(cmd);
        self.redo_stack.clear();
        Ok(())
    }

    pub fn undo(&mut self, db: &Database) -> Result<(), String> {
        if let Some(mut cmd) = self.undo_stack.pop() {
            cmd.undo(db)?;
            self.redo_stack.push(cmd);
            Ok(())
        } else {
            Err("No commands to undo".to_string())
        }
    }

    pub fn redo(&mut self, db: &Database) -> Result<(), String> {
        if let Some(mut cmd) = self.redo_stack.pop() {
            cmd.execute(db)?;
            self.undo_stack.push(cmd);
            Ok(())
        } else {
            Err("No commands to redo".to_string())
        }
    }
}
