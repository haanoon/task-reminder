//! Task editor dialog widget.
//!
//! A clean, elegant modal editor built with Libadwaita preferences/action rows.

use crate::db::{Database, Priority, Task};
use adw::prelude::*;
use gtk::glib;
use chrono::Datelike;
use std::rc::Rc;

pub struct TaskEditor {
    dialog: adw::AlertDialog,
    title_entry: gtk::Entry,
    notes_buffer: gtk::TextBuffer,
    priority_row: adw::ComboRow,
    due_calendar: gtk::Calendar,
    has_due_switch: gtk::Switch,
}

impl TaskEditor {
    pub fn new(parent: &gtk::Window, database: Rc<Database>, task: Option<Task>, on_save: impl Fn() + 'static) -> Rc<Self> {
        let is_edit = task.is_some();
        let dialog = adw::AlertDialog::builder()
            .heading(if is_edit { "Modify Task Details" } else { "Create New Task" })
            .build();

        // Root box
        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        content_box.set_margin_bottom(12);

        // Title Entry
        let title_label = gtk::Label::new(Some("Title"));
        title_label.set_halign(gtk::Align::Start);
        title_label.add_css_class("dim-label");
        let title_entry = gtk::Entry::builder()
            .placeholder_text("Task title")
            .build();
        title_entry.add_css_class("editor-title-entry");
        
        content_box.append(&title_label);
        content_box.append(&title_entry);

        // Notes (Multiline Text View)
        let notes_label = gtk::Label::new(Some("Notes"));
        notes_label.set_halign(gtk::Align::Start);
        notes_label.add_css_class("dim-label");
        
        let notes_scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .min_content_height(80)
            .propagate_natural_height(true)
            .build();
        notes_scroll.add_css_class("editor-notes-scroll");

        let notes_view = gtk::TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .accepts_tab(false)
            .build();
        notes_view.add_css_class("editor-notes-view");
        let notes_buffer = notes_view.buffer();
        notes_scroll.set_child(Some(&notes_view));

        content_box.append(&notes_label);
        content_box.append(&notes_scroll);

        // Preferences Group for Priority and Date
        let pref_group = adw::PreferencesGroup::new();
        pref_group.set_margin_top(8);

        // Priority Row
        let priority_row = adw::ComboRow::builder()
            .title("Priority")
            .model(&gtk::StringList::new(&["None", "Low", "Medium", "High"]))
            .build();
        pref_group.add(&priority_row);

        // Due Date Toggle Row
        let due_toggle_row = adw::ActionRow::builder()
            .title("Set Due Date")
            .build();
        let has_due_switch = gtk::Switch::builder()
            .valign(gtk::Align::Center)
            .build();
        due_toggle_row.add_suffix(&has_due_switch);
        pref_group.add(&due_toggle_row);

        content_box.append(&pref_group);

        // Calendar Dropdown/Section
        let calendar_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        calendar_box.set_visible(false);
        calendar_box.set_margin_top(4);

        let due_calendar = gtk::Calendar::new();
        due_calendar.add_css_class("editor-calendar");
        calendar_box.append(&due_calendar);

        let clear_due_btn = gtk::Button::builder()
            .label("Clear Date")
            .halign(gtk::Align::End)
            .build();
        clear_due_btn.add_css_class("destructive-action");
        calendar_box.append(&clear_due_btn);

        content_box.append(&calendar_box);

        dialog.set_extra_child(Some(&content_box));

        // Setup actions
        dialog.add_response("cancel", "Cancel");
        dialog.add_response("save", if is_edit { "Save" } else { "Create" });
        dialog.set_default_response(Some("save"));
        dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);

        // Populate fields if editing
        if let Some(ref t) = task {
            title_entry.set_text(&t.title);
            notes_buffer.set_text(&t.notes);
            priority_row.set_selected(t.priority as u32);
            
            if let Some(ref date_str) = t.due_date {
                if let Ok(parsed) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    has_due_switch.set_active(true);
                    calendar_box.set_visible(true);
                    
                    let glib_date = glib::DateTime::from_local(
                        parsed.year(),
                        parsed.month() as i32,
                        parsed.day() as i32,
                        0, 0, 0.0
                    ).unwrap();
                    due_calendar.select_day(&glib_date);
                }
            }
        }

        // Switch toggles calendar visibility
        has_due_switch.connect_active_notify({
            let cal_box = calendar_box.clone();
            move |sw| {
                cal_box.set_visible(sw.is_active());
            }
        });

        // Clear button resets switch and hides calendar
        clear_due_btn.connect_clicked({
            let sw = has_due_switch.clone();
            move |_| {
                sw.set_active(false);
            }
        });

        let editor = Rc::new(Self {
            dialog: dialog.clone(),
            title_entry,
            notes_buffer,
            priority_row,
            due_calendar,
            has_due_switch,
        });

        // Handle responses
        dialog.connect_response(None, {
            let editor = editor.clone();
            let db = database;
            let task_id = task.map(|t| t.id);
            move |_, response| {
                if response == "save" {
                    let title = editor.title_entry.text().trim().to_string();
                    if title.is_empty() {
                        return;
                    }
                    
                    let notes = editor.notes_buffer.text(
                        &editor.notes_buffer.start_iter(),
                        &editor.notes_buffer.end_iter(),
                        false
                    ).to_string();

                    let priority = Priority::from_i32(editor.priority_row.selected() as i32);

                    let due_date = if editor.has_due_switch.is_active() {
                        let dt = editor.due_calendar.date();
                        Some(format!("{}-{:02}-{:02}", dt.year(), dt.month(), dt.day_of_month()))
                    } else {
                        None
                    };

                    if let Some(ref id) = task_id {
                        db.update_task(id, &title, &notes, priority, due_date.as_deref()).ok();
                    } else {
                        // Creating a new task
                        let lists = db.get_lists().unwrap_or_default();
                        if !lists.is_empty() {
                            // Default to first list (Inbox)
                            let list_id = &lists[0].id;
                            if let Ok(new_task) = db.create_task(list_id, &title) {
                                db.update_task(&new_task.id, &title, &notes, priority, due_date.as_deref()).ok();
                            }
                        }
                    }
                    on_save();
                }
            }
        });

        dialog.present(Some(parent));
        editor
    }
}
