use std::str::FromStr;

use clap::{ArgAction, Parser};
use clap_serde_derive::ClapSerde;
use serde::{Deserialize, Serialize};

pub mod color;
pub use color::Color;

mod macros;
use macros::make_default_value_getter;

#[cfg(feature = "build-time")]
pub mod imp;

#[cfg(feature = "build-time")]
pub use imp::CONFIG;
#[cfg(feature = "build-time")]
pub use imp::parse_cli_args_with_config;

/// Display images from files.
///
/// Example usage: `macrotis example1.png example2.svg`
#[derive(Parser)]
#[clap(author, version, about)]
#[cfg_attr(feature = "build-time", clap(styles = crate::config::imp::get_styles()))]
struct ArgsWithConfig {
    /// Path to image files. Can be relative to the current work directory or absolute.
    /// If '-' is provided as argument, the paths will be read from stdin, separated by newlines.
    images: Vec<std::path::PathBuf>,

    /// Path to the config file. Defaults to ~/.config/macrotis/macrotis.toml
    #[clap(short, long = "config")]
    config_path: Option<std::path::PathBuf>,

    /// Arguments that are configurable via the config file as well
    #[command(flatten)]
    config: <Config as ClapSerde>::Opt,
}

#[derive(ClapSerde, Serialize, Deserialize, Debug)]
pub struct Config {
    /// Whether to make the window transparent.
    #[arg(short = "t", long, action = ArgAction::SetTrue)]
    #[serde(default)]
    pub no_transparency: bool,
    /// Whether to open the app in fullscreen mode.
    #[arg(short, long, action = ArgAction::SetTrue)]
    #[serde(default)]
    pub fullscreen: bool,
    /// App ID - specifically useful for styling the app's window via desktop environments.
    #[arg(long)]
    #[serde(default)]
    pub app_id: Option<String>,
    /// Theme config.
    #[command(flatten)]
    #[serde(default)]
    pub theme: ThemeConfig,
    /// Key bindings.
    #[arg(skip)]
    #[serde(default)]
    pub keybindings: Vec<KeyBinding>,
}

#[derive(Serialize, Deserialize, Debug, clap::Args)]
pub struct ThemeConfig {
    /// Background color.
    #[arg(long = "background-color", group = "theme", default_value_t = ThemeConfig::background_default())]
    #[serde(default = "ThemeConfig::background_default")]
    pub background: Color,
    /// Foreground (text) color.
    #[arg(long = "foreground-color", default_value_t = ThemeConfig::foreground_default())]
    #[serde(default = "ThemeConfig::foreground_default")]
    pub foreground: Color,

    /// Surface color.
    #[arg(long = "surface-color", default_value_t = ThemeConfig::surface_default())]
    #[serde(default = "ThemeConfig::surface_default")]
    pub surface: Color,
    /// Primary color.
    #[arg(long = "primary-color", default_value_t = ThemeConfig::primary_default())]
    #[serde(default = "ThemeConfig::primary_default")]
    pub primary: Color,

    /// Error color.
    #[arg(long = "error-color", default_value_t = ThemeConfig::error_default())]
    #[serde(default = "ThemeConfig::error_default")]
    pub error: Color,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        // Default to Catppuccin Mocha colors, see https://catppuccin.com/palette/
        Self {
            background: Color::from_str("#1e1e2e").unwrap(),
            foreground: Color::from_str("#cdd6f4").unwrap(),
            surface: Color::from_str("#6c708688").unwrap(),
            primary: Color::from_str("#cba6f7").unwrap(),
            error: Color::from_str("#d20f39").unwrap(),
        }
    }
}

/// A key binding consisting of a keyboard shortcut and a [gpui::Action].
#[derive(Serialize, Deserialize, clap::Args, Debug, Clone)]
pub struct KeyBinding {
    /// The keyboard shortcut that should trigger [KeyBinding::action].
    ///
    /// Multiple keys have to be separated by '-', e.g. 'ctrl-a'.
    ///
    /// Important keys:
    /// - 'ctrl', 'shift', 'alt', 'super'
    /// - 'left', 'right', 'up', 'down'
    pub key: String,
    /// The action to be triggered by the keyboard shortcut.
    ///
    /// Must be one of the ones defined in [mod@crate::actions].
    pub action: String,
}

// Automatically generate default getter methods to be used with #[serde(default = ...)]
// and #[arg(default_value_t = ...)]
//
// This avoids having to re-define the default theme colors in multiple places.
//
// Could possibly be removed if https://github.com/clap-rs/clap/issues/3116 is implemented
make_default_value_getter!(ThemeConfig, background, Color);
make_default_value_getter!(ThemeConfig, foreground, Color);
make_default_value_getter!(ThemeConfig, surface, Color);
make_default_value_getter!(ThemeConfig, primary, Color);
make_default_value_getter!(ThemeConfig, error, Color);
