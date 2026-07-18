mod config;
mod db;
mod style;
mod window;
mod commands;
mod notifications;
#[path = "window/editor.rs"]
mod editor;
#[path = "window/sidebar.rs"]
mod sidebar;
#[path = "window/search.rs"]
mod search;
#[path = "window/settings.rs"]
mod settings;

use adw::prelude::*;
use gtk::{gdk, gio};

/// D-Bus application ID — used for single-instance enforcement and IPC.
const APP_ID: &str = "io.github.wallpapertasks";

fn main() {
    // Initialise structured logging (controlled via RUST_LOG env var)
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("wallpaper_tasks=info"),
    )
    .init();

    log::info!("Wallpaper Tasks v{}", env!("CARGO_PKG_VERSION"));

    let app = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    // ── Startup: load CSS, force dark theme, spawn notifications ──────
    app.connect_startup(|_| {
        load_css();

        let sm = adw::StyleManager::default();
        sm.set_color_scheme(adw::ColorScheme::ForceDark);

        // Launch background notification scheduler
        notifications::spawn(config::Config::database_path());
    });

    // ── Command-line: toggle visibility from a second process ─────────
    //
    // Usage:
    //   wallpaper-tasks            — start / bring to front
    //   wallpaper-tasks --toggle   — show ↔ hide
    //
    // Hyprland keybind example:
    //   bind = SUPER, T, exec, wallpaper-tasks --toggle
    app.connect_command_line(|app, cmdline| {
        let args: Vec<String> = cmdline
            .arguments()
            .iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();

        if args.iter().any(|a| a == "--toggle") {
            let windows = app.windows();
            if let Some(win) = windows.first() {
                let visible = win.is_visible();
                win.set_visible(!visible);
                log::info!("Toggled visibility: {} → {}", visible, !visible);
            } else {
                // First launch with --toggle — just show normally
                app.activate();
            }
        } else {
            app.activate();
        }

        0.into() // exit code for the remote caller
    });

    // ── Activate: build UI (only once) ────────────────────────────────
    app.connect_activate(|app| {
        if !app.windows().is_empty() {
            // Window already exists — make sure it's visible
            if let Some(win) = app.windows().first() {
                win.set_visible(true);
            }
            return;
        }

        let config = config::Config::load();
        window::build_ui(app, &config);
    });

    app.run();
}

/// Load the application CSS and apply it globally.
fn load_css() {
    let provider = gtk::CssProvider::new();
    let dynamic_css = style::get_dynamic_css();
    provider.load_from_data(&dynamic_css);

    let display = gdk::Display::default().expect("Could not connect to a display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Watch for Pywal color changes and reload automatically
    if let Some(home) = dirs::home_dir() {
        let wal_path = home.join(".cache/wal/colors.json");
        let file = gio::File::for_path(wal_path);
        
        if let Ok(monitor) = file.monitor_file(gio::FileMonitorFlags::NONE, gio::Cancellable::NONE) {
            let provider_clone = provider.clone();
            monitor.connect_changed(move |_, _, _, event| {
                if event == gio::FileMonitorEvent::Changed || event == gio::FileMonitorEvent::Created {
                    log::info!("Wallpaper colors changed, reloading CSS...");
                    let updated_css = style::get_dynamic_css();
                    provider_clone.load_from_data(&updated_css);
                }
            });
            // Leak the monitor so it stays alive for the duration of the app
            Box::leak(Box::new(monitor));
        }
    }

    log::info!("Loaded application CSS");
}
