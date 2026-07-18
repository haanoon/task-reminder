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
    pub history: crate::commands::CommandHistory,
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
        db: db.open_again().expect("Failed to open db connection"),
        lists: lists.clone(),
        current_list_idx: 0,
        history: crate::commands::CommandHistory::new(),
    }));

    // ── Window ────────────────────────────────────────────────────────
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Wallpaper Tasks")
        .default_width(config.layout.width)
        .build();

    setup_layer_shell(&window, config);

    // ── Root container using AdwNavigationSplitView or overlay for sliding sidebar ──
    let split_view = adw::NavigationSplitView::new();
    split_view.set_min_sidebar_width(240.0);
    split_view.set_max_sidebar_width(280.0);
    split_view.set_collapsed(true); // sidebar hidden by default

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    main_box.add_css_class("main-container");
    main_box.set_vexpand(true);

    // Sidebar creation
    let sidebar_widget = crate::sidebar::Sidebar::new(
        Rc::new(db.open_again().unwrap()),
        lists.clone(),
        0,
        {
            let state = state.clone();
            let win = window.clone();
            move |idx| {
                state.borrow_mut().current_list_idx = idx;
                fire_action(&win, "refresh-tasks");
            }
        },
        {
            let state = state.clone();
            let win = window.clone();
            move |name| {
                state.borrow().db.create_list(&name, "📋").ok();
                fire_action(&win, "refresh-lists");
            }
        },
        {
            let state = state.clone();
            let win = window.clone();
            move |list_id| {
                state.borrow().db.delete_list(&list_id).ok();
                fire_action(&win, "refresh-lists");
            }
        }
    );

    let sidebar_nav_page = adw::NavigationPage::new(sidebar_widget.widget(), "Lists");
    split_view.set_sidebar(Some(&sidebar_nav_page));

    // ── Header ────────────────────────────────────────────────────────
    let header_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    header_box.add_css_class("header-section");

    // Time greeting & date labels
    let greeting_text = match chrono::Local::now().format("%H").to_string().parse::<u32>() {
        Ok(h) if h < 5 => "Good Night 🌙",
        Ok(h) if h < 12 => "Good Morning ☀️",
        Ok(h) if h < 17 => "Good Afternoon 🌤️",
        Ok(h) if h < 21 => "Good Evening 🌆",
        _ => "Good Night 🌙",
    };
    let title_vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let greeting = gtk::Label::new(Some(greeting_text));
    greeting.add_css_class("greeting");
    greeting.set_halign(gtk::Align::Start);
    let date_str = chrono::Local::now().format("%A, %B %-d").to_string();
    let date = gtk::Label::new(Some(&date_str));
    date.add_css_class("date-label");
    date.set_halign(gtk::Align::Start);
    title_vbox.append(&greeting);
    title_vbox.append(&date);
    title_vbox.set_hexpand(true);

    // Sidebar Toggle button
    let sidebar_toggle_btn = gtk::Button::from_icon_name("view-list-symbolic");
    sidebar_toggle_btn.add_css_class("sidebar-toggle");
    sidebar_toggle_btn.set_tooltip_text(Some("Toggle lists (Ctrl+L)"));
    sidebar_toggle_btn.connect_clicked({
        let sv = split_view.clone();
        move |btn| {
            let collapsed = sv.is_collapsed();
            sv.set_collapsed(!collapsed);
            // Update icon to reflect state
            let icon = if collapsed { "sidebar-show-right-symbolic" } else { "view-list-symbolic" };
            btn.set_icon_name(icon);
        }
    });

    // Search Toggle button
    let search_toggle_btn = gtk::Button::from_icon_name("system-search-symbolic");
    search_toggle_btn.set_tooltip_text(Some("Search tasks (Ctrl+F)"));

    // Settings button
    let settings_btn = gtk::Button::from_icon_name("preferences-system-symbolic");
    settings_btn.set_tooltip_text(Some("Settings (Ctrl+,)"));

    header_box.append(&sidebar_toggle_btn);
    header_box.append(&title_vbox);
    header_box.append(&search_toggle_btn);
    header_box.append(&settings_btn);
    main_box.append(&header_box);

    // Search Bar Widget Overlay
    let state_search = state.clone();
    let win_search = window.clone();
    let search_bar = crate::search::SearchBar::new(
        Rc::new(move || {
            let s = state_search.borrow();
            let mut list_tasks = vec![];
            for l in &s.lists {
                if let Ok(mut t) = s.db.get_tasks(&l.id, false) {
                    list_tasks.append(&mut t);
                }
                if let Ok(mut t) = s.db.get_tasks(&l.id, true) {
                    list_tasks.append(&mut t);
                }
            }
            list_tasks
        }),
        {
            let win = win_search.clone();
            let state = state.clone();
            move |task| {
                // Open editor on selected search task
                let db = Rc::new(state.borrow().db.open_again().unwrap());
                let win_clone = win.clone();
                crate::editor::TaskEditor::new(win.upcast_ref(), db, Some(task), move || {
                    fire_action(&win_clone, "refresh-tasks");
                    fire_action(&win_clone, "refresh-lists");
                });
            }
        }
    );
    main_box.append(search_bar.widget());

    search_toggle_btn.connect_clicked({
        let win = window.clone();
        move |_| {
            fire_action(&win, "toggle-search");
        }
    });

    // ── Task content area ─────────────────────────────────────────────
    let task_content = gtk::Box::new(gtk::Orientation::Vertical, 0);

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

    let content_nav_page = adw::NavigationPage::new(&main_box, "Tasks");
    split_view.set_content(Some(&content_nav_page));
    window.set_child(Some(&split_view));

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
        let win = window.clone();
        move |_, _| {
            refresh_task_display(&state, &alb, &clb, &cs, &ct, &cr, &eb, &win);
        }
    });
    window.add_action(&act_refresh_tasks);

    // ── win.refresh-lists ─────────────────────────────────────────────
    let act_refresh_lists = gio::SimpleAction::new("refresh-lists", None);
    act_refresh_lists.connect_activate({
        let state = state.clone();
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
            // Trigger refresh lists on sidebar if needed (handled by binding state update)
            fire_action(&win, "refresh-tasks");
        }
    });
    window.add_action(&act_refresh_lists);

    // ── win.show-add-task ─────────────────────────────────────────────
    let act_add_task = gio::SimpleAction::new("show-add-task", None);
    act_add_task.connect_activate({
        let state = state.clone();
        let win = window.clone();
        move |_, _| {
            let db = Rc::new(state.borrow().db.open_again().unwrap_or_else(|_| {
                let db_path = Config::database_path();
                Database::open(&db_path).unwrap()
            }));
            let win_clone = win.clone();
            crate::editor::TaskEditor::new(win.upcast_ref(), db, None, move || {
                fire_action(&win_clone, "refresh-tasks");
                fire_action(&win_clone, "refresh-lists");
            });
        }
    });
    window.add_action(&act_add_task);

    // ── win.undo ──────────────────────────────────────────────────────
    let act_undo = gio::SimpleAction::new("undo", None);
    act_undo.connect_activate({
        let state = state.clone();
        let win = window.clone();
        move |_, _| {
            let mut s = state.borrow_mut();
            let db = s.db.open_again().unwrap();
            if let Err(e) = s.history.undo(&db) {
                log::warn!("Undo failed: {}", e);
            } else {
                fire_action(&win, "refresh-tasks");
                fire_action(&win, "refresh-lists");
            }
        }
    });
    window.add_action(&act_undo);

    // ── win.redo ──────────────────────────────────────────────────────
    let act_redo = gio::SimpleAction::new("redo", None);
    act_redo.connect_activate({
        let state = state.clone();
        let win = window.clone();
        move |_, _| {
            let mut s = state.borrow_mut();
            let db = s.db.open_again().unwrap();
            if let Err(e) = s.history.redo(&db) {
                log::warn!("Redo failed: {}", e);
            } else {
                fire_action(&win, "refresh-tasks");
                fire_action(&win, "refresh-lists");
            }
        }
    });
    window.add_action(&act_redo);

    // ── win.toggle-search ─────────────────────────────────────────────
    let act_toggle_search = gio::SimpleAction::new("toggle-search", None);
    act_toggle_search.connect_activate({
        let sb = search_bar.widget().clone();
        let sb_focus = search_bar.clone();
        move |_, _| {
            let vis = sb.is_visible();
            sb.set_visible(!vis);
            if !vis {
                sb_focus.grab_focus();
            }
        }
    });
    window.add_action(&act_toggle_search);

    // ── win.toggle-sidebar ────────────────────────────────────────────
    let act_toggle_sidebar = gio::SimpleAction::new("toggle-sidebar", None);
    act_toggle_sidebar.connect_activate({
        let sv = split_view.clone();
        let btn = sidebar_toggle_btn.clone();
        move |_, _| {
            let collapsed = sv.is_collapsed();
            sv.set_collapsed(!collapsed);
            let icon = if collapsed { "sidebar-show-right-symbolic" } else { "view-list-symbolic" };
            btn.set_icon_name(icon);
        }
    });
    window.add_action(&act_toggle_sidebar);

    // ── win.show-settings ────────────────────────────────────────
    let act_show_settings = gio::SimpleAction::new("show-settings", None);
    act_show_settings.connect_activate({
        let win = window.clone();
        move |_, _| {
            let cfg = crate::config::Config::load();
            crate::settings::show(win.upcast_ref(), cfg, |_new_cfg| {
                // Config saved to disk by the dialog — live reload not wired yet
            });
        }
    });
    window.add_action(&act_show_settings);

    // Set accelerators on Application
    if let Some(app) = window.application() {
        app.set_accels_for_action("win.undo", &["<Control>z"]);
        app.set_accels_for_action("win.redo", &["<Control>y"]);
        app.set_accels_for_action("win.toggle-search", &["<Control>f"]);
        app.set_accels_for_action("win.show-add-task", &["<Control>n"]);
        app.set_accels_for_action("win.toggle-sidebar", &["<Control>l"]);
        app.set_accels_for_action("win.show-settings", &["<Control>comma"]);
    }

    // ══════════════════════════════════════════════════════════════════
    // Signal connections
    // ══════════════════════════════════════════════════════════════════

    // FAB → show add-task dialog
    fab.connect_clicked({
        let win = window.clone();
        move |_| {
            fire_action(&win, "show-add-task");
        }
    });

    // Settings button click
    settings_btn.connect_clicked({
        let win = window.clone();
        move |_| {
            fire_action(&win, "show-settings");
        }
    });

    // ── Window-level Escape → hide the panel (only when no entry is open)
    window.add_controller({
        let win = window.clone();
        let ec = gtk::EventControllerKey::new();
        ec.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
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
    refresh_task_display(
        &state,
        &active_list_box,
        &completed_list_box,
        &completed_section,
        &completed_toggle,
        &completed_revealer,
        &empty_box,
        &window,
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

/// Build a single task row (checkbox · title/details · delete button).
fn build_task_row(task: &Task, state: &Rc<RefCell<AppState>>, window: &gtk::ApplicationWindow) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::builder()
        .activatable(true)
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
    check.set_valign(gtk::Align::Start);
    check.set_margin_top(2);

    // ── Title & Metadata Box ──────────────────────────────────────────
    let text_vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    text_vbox.set_hexpand(true);

    let title = gtk::Label::new(Some(&task.title));
    title.add_css_class("task-title");
    title.set_halign(gtk::Align::Start);
    title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text_vbox.append(&title);

    // Notes preview (first line or truncated text)
    if !task.notes.is_empty() {
        let first_line = task.notes.lines().next().unwrap_or("").trim();
        let notes_lbl = gtk::Label::new(Some(first_line));
        notes_lbl.add_css_class("task-notes-preview");
        notes_lbl.set_halign(gtk::Align::Start);
        notes_lbl.set_ellipsize(gtk::pango::EllipsizeMode::End);
        text_vbox.append(&notes_lbl);
    }

    // Chips box (due date, priority)
    let chips_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    chips_hbox.add_css_class("meta-box");
    let mut has_chips = false;

    if let Some(due_text) = task.due_display() {
        let due_lbl = gtk::Label::new(Some(&due_text));
        due_lbl.add_css_class("due-chip");
        if task.is_overdue() {
            due_lbl.add_css_class("overdue");
        }
        chips_hbox.append(&due_lbl);
        has_chips = true;
    }

    if task.priority != crate::db::Priority::None {
        let prio_lbl = gtk::Label::new(Some(task.priority.label()));
        prio_lbl.add_css_class("priority-badge");
        prio_lbl.add_css_class(task.priority.css_class());
        chips_hbox.append(&prio_lbl);
        has_chips = true;
    }

    if has_chips {
        text_vbox.append(&chips_hbox);
    }

    // ── Delete button ─────────────────────────────────────────────────
    let delete_btn = gtk::Button::from_icon_name("edit-delete-symbolic");
    delete_btn.add_css_class("task-delete");
    delete_btn.set_valign(gtk::Align::Center);
    delete_btn.set_tooltip_text(Some("Delete task"));

    hbox.append(&check);
    hbox.append(&text_vbox);
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

    // ── Row activation (Edit) ─────────────────────────────────────────
    {
        let task_id = task.id.clone();
        let st = state.clone();
        let win = window.clone();
        row.connect_keynav_failed(move |_, _| glib::Propagation::Proceed); // dummy connection to handle activation easily
        // GtkListBox row-activated signal is handled at the ListBox level usually, but we can also use a gesture on the row.
        let click_gesture = gtk::GestureClick::new();
        click_gesture.connect_pressed({
            let task_id = task_id.clone();
            let st = st.clone();
            let win = win.clone();
            move |_, n_press, _, _| {
                if n_press == 2 { // Double click to edit
                    let db = Rc::new(st.borrow().db.open_again().unwrap_or_else(|_| {
                        let db_path = Config::database_path();
                        Database::open(&db_path).unwrap()
                    }));
                    if let Ok(t) = db.get_task(&task_id) {
                        let win_clone = win.clone();
                        crate::editor::TaskEditor::new(win.upcast_ref(), db.clone(), Some(t), move || {
                            fire_action(&win_clone, "refresh-tasks");
                            fire_action(&win_clone, "refresh-lists");
                        });
                    }
                }
            }
        });
        row.add_controller(click_gesture);
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
    window: &gtk::ApplicationWindow,
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
        active_lb.append(&build_task_row(task, state, window));
    }

    // Populate completed tasks
    for task in &completed_tasks {
        completed_lb.append(&build_task_row(task, state, window));
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
