//! Settings dialog widget (Phase 5).
//!
//! Presented as an `adw::PreferencesDialog` (libadwaita 1.6+) with groups for:
//!   - Appearance (opacity, position, width)
//!   - Notifications (enable/disable, morning summary)
//!   - About (version, paths)

use crate::config::Config;
use adw::prelude::*;

/// Show the settings dialog, parented to `parent`.
/// `on_save` is called when the user closes the dialog (changes are saved).
pub fn show(parent: &gtk::Window, config: Config, on_save: impl Fn(Config) + 'static) {
    let dialog = adw::PreferencesDialog::builder()
        .title("Settings")
        .build();

    // ── Appearance ────────────────────────────────────────────────────
    let appearance_group = adw::PreferencesGroup::builder()
        .title("Appearance")
        .build();

    // Position selector
    let position_row = adw::ComboRow::builder()
        .title("Panel position")
        .subtitle("Which screen edge to anchor the panel to")
        .model(&gtk::StringList::new(&["Left", "Right", "Center"]))
        .build();
    let current_pos_idx = match config.layout.position.as_str() {
        "left" => 0u32,
        "center" => 2,
        _ => 1, // "right"
    };
    position_row.set_selected(current_pos_idx);
    appearance_group.add(&position_row);

    // Opacity spin row
    let opacity_row = adw::SpinRow::builder()
        .title("Panel opacity")
        .subtitle("Transparency (0.3 = translucent, 1.0 = opaque)")
        .climb_rate(0.05)
        .digits(2)
        .build();
    opacity_row.set_adjustment(Some(&gtk::Adjustment::new(
        config.appearance.opacity,
        0.3, 1.0, 0.05, 0.1, 0.0,
    )));
    appearance_group.add(&opacity_row);

    // Width spin row
    let width_row = adw::SpinRow::builder()
        .title("Panel width (px)")
        .subtitle("Width of the task panel in pixels")
        .climb_rate(10.0)
        .digits(0)
        .build();
    width_row.set_adjustment(Some(&gtk::Adjustment::new(
        config.layout.width as f64,
        240.0, 800.0, 10.0, 50.0, 0.0,
    )));
    appearance_group.add(&width_row);

    // ── Notifications ──────────────────────────────────────────────────
    let notif_group = adw::PreferencesGroup::builder()
        .title("Notifications")
        .build();

    let notif_row = adw::SwitchRow::builder()
        .title("Due date reminders")
        .subtitle("Notify when tasks are due or overdue")
        .build();
    notif_row.set_active(true);
    notif_group.add(&notif_row);

    let morning_row = adw::SwitchRow::builder()
        .title("Morning summary")
        .subtitle("Daily overview at 08:00")
        .build();
    morning_row.set_active(true);
    notif_group.add(&morning_row);

    // ── About ──────────────────────────────────────────────────────────
    let about_group = adw::PreferencesGroup::builder()
        .title("About")
        .build();

    let version_row = adw::ActionRow::builder()
        .title("Version")
        .subtitle(env!("CARGO_PKG_VERSION"))
        .build();
    about_group.add(&version_row);

    let db_path_str = Config::database_path().to_string_lossy().into_owned();
    let db_path_row = adw::ActionRow::builder()
        .title("Database location")
        .subtitle(db_path_str.as_str())
        .build();
    about_group.add(&db_path_row);

    let cfg_path_str = Config::config_path().to_string_lossy().into_owned();
    let config_path_row = adw::ActionRow::builder()
        .title("Config location")
        .subtitle(cfg_path_str.as_str())
        .build();
    about_group.add(&config_path_row);

    // ── Page assembly ──────────────────────────────────────────────────
    let page = adw::PreferencesPage::builder()
        .icon_name("preferences-system-symbolic")
        .title("General")
        .build();
    page.add(&appearance_group);
    page.add(&notif_group);
    page.add(&about_group);
    dialog.add(&page);

    // ── Save on close ──────────────────────────────────────────────────
    dialog.connect_closed({
        let base = config.clone();
        move |_| {
            let mut new_cfg = base.clone();
            new_cfg.layout.position = match position_row.selected() {
                0 => "left".into(),
                2 => "center".into(),
                _ => "right".into(),
            };
            new_cfg.appearance.opacity = opacity_row.value();
            new_cfg.layout.width = width_row.value() as i32;
            new_cfg.save();
            on_save(new_cfg);
        }
    });

    dialog.present(Some(parent));
}
