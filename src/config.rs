use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level application configuration, loaded from TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub appearance: Appearance,
    pub layout: Layout,
}

/// Visual appearance settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    /// Color scheme: "dark" or "light".
    pub theme: String,
    /// Window opacity (0.0–1.0). Applied via CSS alpha on the container.
    pub opacity: f64,
}

/// Spatial layout settings for the layer-shell surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Layout {
    /// Which screen edge to anchor: "left", "right", or "center".
    pub position: String,
    /// Width of the task panel in pixels.
    pub width: i32,
    /// Margin from the top edge in pixels.
    pub margin_top: i32,
    /// Margin from the bottom edge in pixels.
    pub margin_bottom: i32,
    /// Margin from the right edge in pixels.
    pub margin_right: i32,
    /// Margin from the left edge in pixels.
    pub margin_left: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            appearance: Appearance::default(),
            layout: Layout::default(),
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
            opacity: 0.88,
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            position: "right".into(),
            width: 380,
            margin_top: 16,
            margin_bottom: 16,
            margin_right: 16,
            margin_left: 16,
        }
    }
}

impl Config {
    /// Directory for configuration files.
    /// Typically `~/.config/wallpaper-tasks/`.
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("wallpaper-tasks")
    }

    /// Full path to the configuration file.
    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    /// Directory for application data (database, backups).
    /// Typically `~/.local/share/wallpaper-tasks/`.
    pub fn data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("wallpaper-tasks")
    }

    /// Full path to the SQLite database file.
    pub fn database_path() -> PathBuf {
        Self::data_dir().join("tasks.db")
    }

    /// Load configuration from disk, falling back to defaults if the file
    /// doesn't exist or is malformed. Creates a default config file on first run.
    pub fn load() -> Self {
        let path = Self::config_path();

        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Config>(&content) {
                    Ok(config) => {
                        log::info!("Loaded config from {}", path.display());
                        return config;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse config (using defaults): {}", e);
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read config (using defaults): {}", e);
                }
            }
        }

        let config = Config::default();
        config.save();
        config
    }

    /// Persist the current configuration to disk.
    pub fn save(&self) {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::warn!("Failed to create config directory: {}", e);
                return;
            }
        }

        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&path, &content) {
                    log::warn!("Failed to write config: {}", e);
                } else {
                    log::info!("Saved config to {}", path.display());
                }
            }
            Err(e) => {
                log::warn!("Failed to serialize config: {}", e);
            }
        }
    }
}
