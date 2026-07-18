//! Main window — layer-shell surface with the full task-management UI.
//!
//! Widget tree (simplified):
//! ```text
//! ApplicationWindow [transparent, layer=Background]
//!  └─ Box.main-container (V)
//!      ├─ Box.header-section (V)  — greeting + date
//!      ├─ ScrolledWindow (H)      — list tabs
//!      │   └─ Box.list-tabs (H)   — [Inbox] [Personal] [Work] [+]
//!      ├─ Revealer                — inline new-list entry
//!      ├─ Separator
//!      └─ Overlay (expand)
//!          ├─ ScrolledWindow (V)
//!          │   └─ Box (V)
//!          │       ├─ Revealer     — inline add-task entry
//!          │       ├─ ListBox      — active tasks
//!          │       ├─ Box          — completed section (toggle + revealer)
//!          │       └─ Box          — empty state
//!          └─ Button.fab           — floating "+" button
//! ```

use crate::config::Config;
use crate::db::{Database, Task};
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use std::cell::RefCell;
use std::rc::Rc;

// ═══════════════════════════════════════════════════════════════════════
// Shared state
// ═══════════════════════════════════════════════════════════════════════

/// Mutable application state shared across GTK callbacks via `Rc<RefCell<…>>`.
pub struct AppState {
    pub db: Database,
    pub lists: Vec<crate::db::TaskList>,
    pub current_list_idx: usize,
}

// ═══════════════════════════════════════════════════════════════════════
// Public entry point
// ═══════════════════════════════════════════════════════════════════════

/// Construct the entire UI and present the layer-shell window.
pub fn build_ui(app: &adw::Application, config: &Config) {
    // ── Database ──────────────────────────────────────────────────────
    let db_path = Config::database_path();
    let db = Database::open(&db_path).expect("Failed to open database");
    let lists = db.get_lists().unwrap_or_default();

    let state = Rc::new(RefCell::new(AppState {
        db,
        lists,
        current_list_idx: 0,
    }));

    // ── Window ────────────────────────────────────────────────────────
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Wallpaper Tasks")
        .default_width(config.layout.width)
        .build();

    setup_layer_shell(&window, config);

    // ── Root container ────────────────────────────────────────────────
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.add_css_class("main-container");
    main_box.set_vexpand(true);

    // ── Header ────────────────────────────────────────────────────────
    main_box.append(&build_header());

    // ── List tabs ─────────────────────────────────────────────────────
    let list_tabs_scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .build();
    let list_tabs_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    list_tabs_box.add_css_class("list-tabs");
    list_tabs_scroll.set_child(Some(&list_tabs_box));
    main_box.append(&list_tabs_scroll);

    // New-list inline entry (initially hidden)
    let new_list_revealer = gtk::Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideDown)
        .transition_duration(150)
        .reveal_child(false)
        .build();
    let new_list_entry = gtk::Entry::builder()
        .placeholder_text("List name…")
        .build();
    new_list_entry.add_css_class("new-list-entry");
    new_list_revealer.set_child(Some(&new_list_entry));
    main_box.append(&new_list_revealer);

    // ── Separator ─────────────────────────────────────────────────────
    let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
    sep.add_css_class("task-separator");
    main_box.append(&sep);

    // ── Task content area ─────────────────────────────────────────────
    let task_content = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Inline add-task entry (initially hidden)
    let add_task_revealer = gtk::Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideDown)
        .transition_duration(150)
        .reveal_child(false)
        .build();
    let add_task_entry = gtk::Entry::builder()
        .placeholder_text("What needs to be done?")
        .build();
    add_task_entry.add_css_class("add-task-entry");
    add_task_revealer.set_child(Some(&add_task_entry));
    task_content.append(&add_task_revealer);

    // Active tasks ListBox
    let active_list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    active_list_box.add_css_class("task-list");
    task_content.append(&active_list_box);

    // Completed section
    let completed_section = gtk::Box::new(gtk::Orientation::Vertical, 0);
    completed_section.set_visible(false);

    let completed_toggle = gtk::Button::with_label("▾ Completed (0)");
    completed_toggle.add_css_class("completed-toggle");
    completed_toggle.set_halign(gtk::Align::Start);
    completed_section.append(&completed_toggle);

    let completed_revealer = gtk::Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideDown)
        .transition_duration(200)
        .reveal_child(false)
        .build();
    let completed_list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    completed_list_box.add_css_class("task-list");
    completed_revealer.set_child(Some(&completed_list_box));
    completed_section.append(&completed_revealer);

    task_content.append(&completed_section);

    // Empty state
    let empty_box = build_empty_state();
    task_content.append(&empty_box);

    // Scrolled window wrapping the task content
    let task_scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .build();
    task_scroll.set_child(Some(&task_content));

    // Overlay: tasks underneath, FAB on top
    let overlay = gtk::Overlay::new();
    overlay.set_child(Some(&task_scroll));
    overlay.set_vexpand(true);

    let fab = gtk::Button::from_icon_name("list-add-symbolic");
    fab.add_css_class("fab");
    fab.set_halign(gtk::Align::End);
    fab.set_valign(gtk::Align::End);
    fab.set_margin_end(20);
    fab.set_margin_bottom(20);
    fab.set_tooltip_text(Some("Add task"));
    overlay.add_overlay(&fab);

    main_box.append(&overlay);
    window.set_child(Some(&main_box));

    // ══════════════════════════════════════════════════════════════════
    // Actions — decouple widget callbacks from refresh logic
    // ══════════════════════════════════════════════════════════════════

    // ── win.refresh-tasks ─────────────────────────────────────────────
    let act_refresh_tasks = gio::SimpleAction::new("refresh-tasks", None);
    act_refresh_tasks.connect_activate({
        let state = state.clone();
        let alb = active_list_box.clone();
        let clb = completed_list_box.clone();
        let cs = completed_section.clone();
        let ct = completed_toggle.clone();
        let cr = completed_revealer.clone();
        let eb = empty_box.clone();
        move |_, _| {
            refresh_task_display(&state, &alb, &clb, &cs, &ct, &cr, &eb);
        }
    });
    window.add_action(&act_refresh_tasks);

    // ── win.refresh-lists ─────────────────────────────────────────────
    let act_refresh_lists = gio::SimpleAction::new("refresh-lists", None);
    act_refresh_lists.connect_activate({
        let state = state.clone();
        let ltb = list_tabs_box.clone();
        let win = window.clone();
        move |_, _| {
            // Reload list data from the database
            {
                let mut s = state.borrow_mut();
                s.lists = s.db.get_lists().unwrap_or_default();
                if s.current_list_idx >= s.lists.len() {
                    s.current_list_idx = s.lists.len().saturating_sub(1);
                }
            }
            refresh_list_tabs(&state, &ltb, &win);
            // Cascade: also refresh the task view
            fire_action(&win, "refresh-tasks");
        }
    });
    window.add_action(&act_refresh_lists);

    // ── win.show-new-list ─────────────────────────────────────────────
    let act_new_list = gio::SimpleAction::new("show-new-list", None);
    act_new_list.connect_activate({
        let rev = new_list_revealer.clone();
        let ent = new_list_entry.clone();
        move |_, _| {
            rev.set_reveal_child(true);
            ent.grab_focus();
        }
    });
    window.add_action(&act_new_list);

    // ── win.show-add-task ─────────────────────────────────────────────
    let act_add_task = gio::SimpleAction::new("show-add-task", None);
    act_add_task.connect_activate({
        let rev = add_task_revealer.clone();
        let ent = add_task_entry.clone();
        move |_, _| {
            rev.set_reveal_child(true);
            ent.grab_focus();
        }
    });
    window.add_action(&act_add_task);

    // ══════════════════════════════════════════════════════════════════
    // Signal connections
    // ══════════════════════════════════════════════════════════════════

    // FAB → show add-task entry
    fab.connect_clicked({
        let win = window.clone();
        move |_| {
            fire_action(&win, "show-add-task");
        }
    });

    // Add-task entry → Enter creates a task
    add_task_entry.connect_activate({
        let state = state.clone();
        let rev = add_task_revealer.clone();
        let ent = add_task_entry.clone();
        let win = window.clone();
        move |_| {
            let title = ent.text().trim().to_string();
            if title.is_empty() {
                return;
            }
            {
                let s = state.borrow();
                if s.lists.is_empty() {
                    return;
                }
                let list_id = &s.lists[s.current_list_idx].id;
                if let Err(e) = s.db.create_task(list_id, &title) {
                    log::error!("Failed to create task: {e}");
                    return;
                }
            }
            ent.set_text("");
            rev.set_reveal_child(false);
            fire_action(&win, "refresh-tasks");
            fire_action(&win, "refresh-lists");
        }
    });

    // Add-task entry → Escape cancels
    add_task_entry.add_controller(escape_controller({
        let rev = add_task_revealer.clone();
        let ent = add_task_entry.clone();
        move || {
            rev.set_reveal_child(false);
            ent.set_text("");
        }
    }));

    // New-list entry → Enter creates a list
    new_list_entry.connect_activate({
        let state = state.clone();
        let rev = new_list_revealer.clone();
        let ent = new_list_entry.clone();
        let win = window.clone();
        move |_| {
            let name = ent.text().trim().to_string();
            if name.is_empty() {
                return;
            }
            // Create list and switch to it
            let new_idx = {
                let s = state.borrow();
                let list_count = s.lists.len();
                if let Err(e) = s.db.create_list(&name, "📋") {
                    log::error!("Failed to create list: {e}");
                    return;
                }
                list_count // will be the index after refresh
            };
            state.borrow_mut().current_list_idx = new_idx;
            ent.set_text("");
            rev.set_reveal_child(false);
            fire_action(&win, "refresh-lists");
        }
    });

    // New-list entry → Escape cancels
    new_list_entry.add_controller(escape_controller({
        let rev = new_list_revealer.clone();
        let ent = new_list_entry.clone();
        move || {
            rev.set_reveal_child(false);
            ent.set_text("");
        }
    }));

    // Completed toggle → expand / collapse
    completed_toggle.connect_clicked({
        let cr = completed_revealer.clone();
        let ct = completed_toggle.clone();
        move |_| {
            let revealed = cr.reveals_child();
            cr.set_reveal_child(!revealed);
            // Swap arrow character in the label
            if let Some(label) = ct.label() {
                let new_label = if revealed {
                    label.replace('▴', "▾")
                } else {
                    label.replace('▾', "▴")
                };
                ct.set_label(&new_label);
            }
        }
    });

    // Window-level Escape → hide the panel (only when no entry is open)
    window.add_controller({
        let win = window.clone();
        let atr = add_task_revealer.clone();
        let nlr = new_list_revealer.clone();
        let ec = gtk::EventControllerKey::new();
        ec.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape && !atr.reveals_child() && !nlr.reveals_child() {
                win.set_visible(false);
                log::info!("Hidden via Escape");
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        ec
    });

    // ══════════════════════════════════════════════════════════════════
    // Initial render
    // ══════════════════════════════════════════════════════════════════
    refresh_list_tabs(&state, &list_tabs_box, &window);
    refresh_task_display(
        &state,
        &active_list_box,
        &completed_list_box,
        &completed_section,
        &completed_toggle,
        &completed_revealer,
        &empty_box,
    );

    window.present();
    log::info!("Window presented on layer shell");
}

// ═══════════════════════════════════════════════════════════════════════
// Layer shell setup
// ═══════════════════════════════════════════════════════════════════════

/// Attach the window to the wlr-layer-shell Background layer.
fn setup_layer_shell(window: &gtk::ApplicationWindow, config: &Config) {
    use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

    let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
        || std::env::var("XDG_SESSION_TYPE").map_or(false, |v| v == "wayland");

    if !is_wayland {
        log::warn!("Not on Wayland — layer-shell disabled, running as normal window");
        return;
    }

    window.init_layer_shell();
    window.set_layer(Layer::Background);
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_namespace(Some("wallpaper-tasks"));

    // Anchor to the configured edge; always stretch top-to-bottom
    match config.layout.position.as_str() {
        "left" => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, true);
            window.set_anchor(Edge::Right, false);
        }
        "center" => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, false);
            window.set_anchor(Edge::Right, false);
        }
        _ => {
            // "right" (default)
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Bottom, true);
            window.set_anchor(Edge::Left, false);
            window.set_anchor(Edge::Right, true);
        }
    }

    window.set_margin(Edge::Top, config.layout.margin_top);
    window.set_margin(Edge::Bottom, config.layout.margin_bottom);
    window.set_margin(Edge::Left, config.layout.margin_left);
    window.set_margin(Edge::Right, config.layout.margin_right);

    log::info!(
        "Layer shell configured: position={}, layer=Background, keyboard=OnDemand",
        config.layout.position
    );
}

// ═══════════════════════════════════════════════════════════════════════
// Widget builders
// ═══════════════════════════════════════════════════════════════════════

/// Header section: time-of-day greeting and formatted date.
fn build_header() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 2);
    container.add_css_class("header-section");

    let greeting_text = match chrono::Local::now().format("%H").to_string().parse::<u32>() {
        Ok(h) if h < 5 => "Good Night 🌙",
        Ok(h) if h < 12 => "Good Morning ☀️",
        Ok(h) if h < 17 => "Good Afternoon 🌤️",
        Ok(h) if h < 21 => "Good Evening 🌆",
        _ => "Good Night 🌙",
    };

    let greeting = gtk::Label::new(Some(greeting_text));
    greeting.add_css_class("greeting");
    greeting.set_halign(gtk::Align::Start);

    let date_str = chrono::Local::now().format("%A, %B %-d").to_string();
    let date = gtk::Label::new(Some(&date_str));
    date.add_css_class("date-label");
    date.set_halign(gtk::Align::Start);

    container.append(&greeting);
    container.append(&date);
    container
}

/// Placeholder shown when the active list has no tasks.
fn build_empty_state() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 4);
    container.add_css_class("empty-state");
    container.set_halign(gtk::Align::Center);
    container.set_valign(gtk::Align::Center);
    container.set_vexpand(true);
    container.set_visible(false);

    let icon = gtk::Label::new(Some("✨"));
    icon.add_css_class("empty-icon");
    container.append(&icon);

    let title = gtk::Label::new(Some("No tasks yet"));
    title.add_css_class("empty-title");
    container.append(&title);

    let sub = gtk::Label::new(Some("Tap + to add your first task"));
    sub.add_css_class("empty-subtitle");
    container.append(&sub);

    container
}

/// Build a single task row (checkbox · title · delete button).
fn build_task_row(task: &Task, state: &Rc<RefCell<AppState>>) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::builder()
        .activatable(false)
        .selectable(false)
        .build();

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    hbox.add_css_class("task-row-box");
    if task.completed {
        hbox.add_css_class("task-completed");
    }

    // ── Checkbox ──────────────────────────────────────────────────────
    let check = gtk::CheckButton::new();
    check.set_active(task.completed);
    check.set_valign(gtk::Align::Center);

    // ── Title ─────────────────────────────────────────────────────────
    let title = gtk::Label::new(Some(&task.title));
    title.add_css_class("task-title");
    title.set_halign(gtk::Align::Start);
    title.set_hexpand(true);
    title.set_ellipsize(gtk::pango::EllipsizeMode::End);

    // ── Delete button ─────────────────────────────────────────────────
    let delete_btn = gtk::Button::from_icon_name("edit-delete-symbolic");
    delete_btn.add_css_class("task-delete");
    delete_btn.set_valign(gtk::Align::Center);
    delete_btn.set_tooltip_text(Some("Delete task"));

    hbox.append(&check);
    hbox.append(&title);
    hbox.append(&delete_btn);
    row.set_child(Some(&hbox));

    // ── Toggle complete ───────────────────────────────────────────────
    {
        let task_id = task.id.clone();
        let st = state.clone();
        check.connect_toggled(move |btn| {
            st.borrow().db.toggle_task(&task_id).ok();
            btn.activate_action("win.refresh-tasks", None).ok();
            btn.activate_action("win.refresh-lists", None).ok();
        });
    }

    // ── Delete ────────────────────────────────────────────────────────
    {
        let task_id = task.id.clone();
        let st = state.clone();
        delete_btn.connect_clicked(move |btn| {
            st.borrow().db.delete_task(&task_id).ok();
            btn.activate_action("win.refresh-tasks", None).ok();
            btn.activate_action("win.refresh-lists", None).ok();
        });
    }

    row
}

// ═══════════════════════════════════════════════════════════════════════
// Refresh helpers
// ═══════════════════════════════════════════════════════════════════════

/// Rebuild the task ListBoxes from the database for the currently selected list.
fn refresh_task_display(
    state: &Rc<RefCell<AppState>>,
    active_lb: &gtk::ListBox,
    completed_lb: &gtk::ListBox,
    completed_section: &gtk::Box,
    completed_toggle: &gtk::Button,
    completed_revealer: &gtk::Revealer,
    empty_box: &gtk::Box,
) {
    // Fetch data (borrow kept as short as possible)
    let (active_tasks, completed_tasks) = {
        let s = state.borrow();
        if s.lists.is_empty() {
            (vec![], vec![])
        } else {
            let lid = &s.lists[s.current_list_idx].id;
            (
                s.db.get_tasks(lid, false).unwrap_or_default(),
                s.db.get_tasks(lid, true).unwrap_or_default(),
            )
        }
    };

    // Clear existing rows
    clear_list_box(active_lb);
    clear_list_box(completed_lb);

    // Populate active tasks
    for task in &active_tasks {
        active_lb.append(&build_task_row(task, state));
    }

    // Populate completed tasks
    for task in &completed_tasks {
        completed_lb.append(&build_task_row(task, state));
    }

    // Update completed-section visibility and label
    let has_completed = !completed_tasks.is_empty();
    completed_section.set_visible(has_completed);
    if has_completed {
        let arrow = if completed_revealer.reveals_child() {
            "▴"
        } else {
            "▾"
        };
        completed_toggle.set_label(&format!("{arrow} Completed ({})", completed_tasks.len()));
    }

    // Empty state: show only when there are zero tasks of any kind
    let all_empty = active_tasks.is_empty() && completed_tasks.is_empty();
    empty_box.set_visible(all_empty);
    active_lb.set_visible(!active_tasks.is_empty());
}

/// Rebuild the horizontal list-tab buttons.
fn refresh_list_tabs(
    state: &Rc<RefCell<AppState>>,
    tabs_box: &gtk::Box,
    window: &gtk::ApplicationWindow,
) {
    // Remove existing buttons
    while let Some(child) = tabs_box.first_child() {
        tabs_box.remove(&child);
    }

    let s = state.borrow();
    let current = s.current_list_idx;

    for (i, list) in s.lists.iter().enumerate() {
        let count = s.db.task_count(&list.id).unwrap_or(0);
        let label = if count > 0 {
            format!("{} {} ({})", list.icon, list.name, count)
        } else {
            format!("{} {}", list.icon, list.name)
        };

        let btn = gtk::Button::with_label(&label);
        btn.add_css_class("list-tab");
        if i == current {
            btn.add_css_class("active");
        }

        // Switch list on click
        let st = state.clone();
        let win = window.clone();
        btn.connect_clicked(move |_| {
            st.borrow_mut().current_list_idx = i;
            fire_action(&win, "refresh-lists");
        });

        tabs_box.append(&btn);
    }

    // + button for new list
    let add_btn = gtk::Button::from_icon_name("list-add-symbolic");
    add_btn.add_css_class("add-list-btn");
    add_btn.set_tooltip_text(Some("New list"));
    add_btn.connect_clicked({
        let win = window.clone();
        move |_| {
            fire_action(&win, "show-new-list");
        }
    });
    tabs_box.append(&add_btn);
}

// ═══════════════════════════════════════════════════════════════════════
// Utilities
// ═══════════════════════════════════════════════════════════════════════

/// Remove every child from a `ListBox`.
fn clear_list_box(lb: &gtk::ListBox) {
    while let Some(child) = lb.first_child() {
        lb.remove(&child);
    }
}

/// Activate a named action on the window, avoiding the WidgetExt / ActionGroupExt
/// ambiguity that arises because `ApplicationWindow` implements both traits.
fn fire_action(window: &gtk::ApplicationWindow, name: &str) {
    gio::prelude::ActionGroupExt::activate_action(window, name, None);
}

/// Create an `EventControllerKey` that runs `action` on Escape and stops
/// propagation.
fn escape_controller(action: impl Fn() + 'static) -> gtk::EventControllerKey {
    let ec = gtk::EventControllerKey::new();
    ec.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            action();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    ec
}
