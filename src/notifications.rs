//! Background notification scheduler.
//!
//! Spawns a native OS thread that wakes up every `CHECK_INTERVAL_SECS` seconds,
//! queries the database for tasks that are due today or overdue, and fires
//! desktop notifications via D-Bus (notify-rust).
//!
//! Design constraints:
//! - Must NOT touch any GTK objects (all GTK work is on the main thread).
//! - Opens its own SQLite connection (rusqlite is not Send).
//! - Tracks which notification IDs it has already sent to avoid spam.

use crate::db::Database;
use chrono::Local;
use notify_rust::{Notification, Timeout};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

/// How often to check for due tasks (seconds).
const CHECK_INTERVAL_SECS: u64 = 300; // 5 minutes

/// Send a morning summary of today's tasks (once per day, around 08:00).
const MORNING_HOUR: u32 = 8;

/// Spawn the notification scheduler thread.
///
/// This function returns immediately; the thread runs for the lifetime of
/// the process.  Errors inside the thread are logged but not propagated.
pub fn spawn(db_path: std::path::PathBuf) {
    thread::Builder::new()
        .name("notif-scheduler".into())
        .spawn(move || {
            log::info!("[notif] Notification scheduler started");
            run_loop(db_path);
        })
        .expect("Failed to spawn notification thread");
}

fn run_loop(db_path: std::path::PathBuf) {
    let mut already_notified: HashSet<String> = HashSet::new();
    let mut morning_summary_sent_date: Option<chrono::NaiveDate> = None;

    loop {
        let db = match Database::open(&db_path) {
            Ok(d) => d,
            Err(e) => {
                log::warn!("[notif] Failed to open DB: {}", e);
                thread::sleep(Duration::from_secs(CHECK_INTERVAL_SECS));
                continue;
            }
        };

        let today = Local::now().naive_local().date();
        let now_hour = Local::now().hour();

        // ── Morning summary (once per day) ────────────────────────────
        let should_send_morning = now_hour >= MORNING_HOUR
            && morning_summary_sent_date.map_or(true, |d| d < today);

        if should_send_morning {
            send_morning_summary(&db, today, &mut morning_summary_sent_date);
        }

        // ── Per-task overdue / due-today notifications ─────────────────
        check_task_notifications(&db, today, &mut already_notified);

        thread::sleep(Duration::from_secs(CHECK_INTERVAL_SECS));
    }
}

fn send_morning_summary(
    db: &Database,
    today: chrono::NaiveDate,
    sent_date: &mut Option<chrono::NaiveDate>,
) {
    let lists = match db.get_lists() {
        Ok(l) => l,
        Err(_) => return,
    };

    let today_str = today.format("%Y-%m-%d").to_string();
    let mut due_today = 0usize;
    let mut overdue = 0usize;

    for list in &lists {
        if let Ok(tasks) = db.get_tasks(&list.id, false) {
            for task in &tasks {
                if let Some(ref due) = task.due_date {
                    if due == &today_str {
                        due_today += 1;
                    } else if due < &today_str {
                        overdue += 1;
                    }
                }
            }
        }
    }

    if due_today == 0 && overdue == 0 {
        *sent_date = Some(today);
        return;
    }

    let mut body_parts = Vec::new();
    if due_today > 0 {
        body_parts.push(format!("{} task{} due today", due_today, if due_today == 1 { "" } else { "s" }));
    }
    if overdue > 0 {
        body_parts.push(format!("{} overdue", overdue));
    }

    let body = body_parts.join(" · ");
    let _ = Notification::new()
        .summary("Wallpaper Tasks — Good morning!")
        .body(&body)
        .appname("wallpaper-tasks")
        .icon("task-due")
        .timeout(Timeout::Milliseconds(8000))
        .show();

    log::info!("[notif] Morning summary sent: {}", body);
    *sent_date = Some(today);
}

fn check_task_notifications(
    db: &Database,
    today: chrono::NaiveDate,
    already_notified: &mut HashSet<String>,
) {
    let lists = match db.get_lists() {
        Ok(l) => l,
        Err(_) => return,
    };

    let today_str = today.format("%Y-%m-%d").to_string();

    for list in &lists {
        let tasks = match db.get_tasks(&list.id, false) {
            Ok(t) => t,
            Err(_) => continue,
        };

        for task in &tasks {
            let Some(ref due) = task.due_date else { continue };

            // Only notify once per task per run
            let notif_key = format!("{}-{}", task.id, due);
            if already_notified.contains(&notif_key) {
                continue;
            }

            let (summary, body, urgency) = if due < &today_str {
                (
                    "Overdue task",
                    format!("\"{}\" was due on {}", task.title, due),
                    notify_rust::Urgency::Critical,
                )
            } else if due == &today_str {
                (
                    "Task due today",
                    format!("\"{}\" is due today", task.title),
                    notify_rust::Urgency::Normal,
                )
            } else {
                continue; // Future task — skip
            };

            let result = Notification::new()
                .summary(summary)
                .body(&body)
                .appname("wallpaper-tasks")
                .icon("task-due")
                .urgency(urgency)
                .timeout(Timeout::Milliseconds(6000))
                .show();

            match result {
                Ok(_) => {
                    log::info!("[notif] Sent notification: {}", body);
                    already_notified.insert(notif_key);
                }
                Err(e) => {
                    log::warn!("[notif] Failed to send notification: {}", e);
                }
            }
        }
    }
}

// ── Re-export chrono::Timelike so the hour() method is in scope ─────────
use chrono::Timelike;
