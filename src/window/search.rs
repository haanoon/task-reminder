//! Fuzzy Search bar overlay widget.

use crate::db::Task;
use adw::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct SearchBar {
    container: gtk::Box,
    entry: gtk::SearchEntry,
    results_list: gtk::ListBox,
}

impl SearchBar {
    pub fn new(
        all_tasks: Rc<dyn Fn() -> Vec<Task>>,
        on_task_selected: impl Fn(Task) + 'static,
    ) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
        container.add_css_class("search-container");
        container.set_visible(false);

        let entry = gtk::SearchEntry::builder()
            .placeholder_text("Search tasks...")
            .build();
        entry.add_css_class("search-bar-entry");
        container.append(&entry);

        let results_list = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .build();
        results_list.add_css_class("search-results-list");

        let scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .min_content_height(100)
            .max_content_height(250)
            .child(&results_list)
            .build();
        container.append(&scroll);

        let on_selected = Rc::new(on_task_selected);

        entry.connect_search_changed({
            let results_list = results_list.clone();
            let all_tasks = all_tasks.clone();
            let on_selected = on_selected.clone();
            move |se| {
                // Clear old search items
                while let Some(child) = results_list.first_child() {
                    results_list.remove(&child);
                }

                let query = se.text().trim().to_lowercase();
                if query.is_empty() {
                    return;
                }

                let tasks = all_tasks();
                for task in tasks {
                    if task.title.to_lowercase().contains(&query) || task.notes.to_lowercase().contains(&query) {
                        let row = gtk::ListBoxRow::new();
                        let row_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
                        row_box.add_css_class("search-result-row");

                        let title = gtk::Label::new(Some(&task.title));
                        title.add_css_class("search-result-title");
                        title.set_ellipsize(gtk::pango::EllipsizeMode::End);
                        row_box.append(&title);

                        row.set_child(Some(&row_box));
                        results_list.append(&row);

                        let on_selected_clone = on_selected.clone();
                        let task_clone = task.clone();
                        let click = gtk::GestureClick::new();
                        click.connect_pressed(move |_, _, _, _| {
                            on_selected_clone(task_clone.clone());
                        });
                        row.add_controller(click);
                    }
                }
            }
        });

        Self {
            container,
            entry,
            results_list,
        }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }

    pub fn grab_focus(&self) {
        self.entry.grab_focus();
    }
}
