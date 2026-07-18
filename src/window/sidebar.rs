//! List navigation sidebar widget.

use crate::db::{Database, TaskList};
use adw::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sidebar {
    container: gtk::Box,
}

impl Sidebar {
    pub fn new(
        _database: Rc<Database>,
        lists: Vec<TaskList>,
        current_idx: usize,
        on_select: impl Fn(usize) + 'static,
        on_create_list: impl Fn(String) + 'static,
        on_delete_list: impl Fn(String) + 'static,
    ) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
        container.add_css_class("sidebar-container");
        container.set_width_request(240);

        // Sidebar Header
        let header_lbl = gtk::Label::new(Some("My Lists"));
        header_lbl.add_css_class("sidebar-header-title");
        header_lbl.set_halign(gtk::Align::Start);
        header_lbl.set_margin_start(16);
        header_lbl.set_margin_top(16);
        header_lbl.set_margin_bottom(8);
        container.append(&header_lbl);

        // List Selection Box
        let list_box = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .build();
        list_box.add_css_class("sidebar-list");
        
        let on_select = Rc::new(on_select);
        let on_delete_list = Rc::new(on_delete_list);

        for (i, list) in lists.iter().enumerate() {
            let row = gtk::ListBoxRow::new();
            let row_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
            row_box.add_css_class("sidebar-row-box");

            let icon_lbl = gtk::Label::new(Some(&list.icon));
            let name_lbl = gtk::Label::new(Some(&list.name));
            name_lbl.set_hexpand(true);
            name_lbl.set_halign(gtk::Align::Start);

            row_box.append(&icon_lbl);
            row_box.append(&name_lbl);

            // Add delete button for custom lists (not the default seeded ones)
            if i > 2 {
                let delete_btn = gtk::Button::from_icon_name("edit-delete-symbolic");
                delete_btn.add_css_class("sidebar-delete-btn");
                delete_btn.set_tooltip_text(Some("Delete list"));
                
                let list_id = list.id.clone();
                let on_delete = on_delete_list.clone();
                delete_btn.connect_clicked(move |_| {
                    on_delete(list_id.clone());
                });
                row_box.append(&delete_btn);
            }

            row.set_child(Some(&row_box));
            list_box.append(&row);

            if i == current_idx {
                list_box.select_row(Some(&row));
            }
        }

        // Selection Changed Handler
        let on_select_clone = on_select.clone();
        list_box.connect_row_selected(move |_, row| {
            if let Some(r) = row {
                on_select_clone(r.index() as usize);
            }
        });

        // Activation Handler (clicking the row, even if already selected)
        let on_select_clone2 = on_select.clone();
        list_box.connect_row_activated(move |_, row| {
            on_select_clone2(row.index() as usize);
        });

        let scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .child(&list_box)
            .build();
        container.append(&scroll);

        // Add List Inline Entry
        let inline_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        inline_box.set_margin_start(12);
        inline_box.set_margin_end(12);
        inline_box.set_margin_bottom(12);

        let entry = gtk::Entry::builder()
            .placeholder_text("New list...")
            .hexpand(true)
            .build();
        entry.add_css_class("sidebar-add-entry");
        
        let add_btn = gtk::Button::from_icon_name("list-add-symbolic");
        add_btn.add_css_class("sidebar-add-btn");

        inline_box.append(&entry);
        inline_box.append(&add_btn);
        container.append(&inline_box);

        let on_create = Rc::new(on_create_list);
        
        let perform_add = {
            let entry = entry.clone();
            let on_create = on_create.clone();
            move || {
                let name = entry.text().trim().to_string();
                if !name.is_empty() {
                    on_create(name);
                    entry.set_text("");
                }
            }
        };

        entry.connect_activate({
            let perform_add = perform_add.clone();
            move |_| { perform_add(); }
        });

        add_btn.connect_clicked(move |_| {
            perform_add();
        });

        Self { container }
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.container
    }
}
