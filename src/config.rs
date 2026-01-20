use std::{
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
};

use clap::{ArgAction, Parser};
use clap_serde_derive::ClapSerde;
use gpui::Action;
use serde::{Deserialize, Serialize};

use crate::{actions::*, color::Color, macros};

pub static CONFIG: OnceLock<Config> = OnceLock::new();
static CONFIG_FILE_NAME: &str = "config";

/// Display images from files.
///
/// Example usage: `macrotis example1.png example2.svg`
#[derive(Parser)]
#[clap(author, version, about, styles = get_styles())]
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
    #[arg(short, long, action = ArgAction::SetTrue)]
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

// Automatically generate default getter methods to be used with #[serde(default = ...)]
// and #[arg(default_value_t = ...)]
//
// This avoids having to re-define the default theme colors in multiple places.
//
// Could possibly be removed if https://github.com/clap-rs/clap/issues/3116 is implemented
macros::make_default_value_getter!(ThemeConfig, background, Color);
macros::make_default_value_getter!(ThemeConfig, foreground, Color);
macros::make_default_value_getter!(ThemeConfig, surface, Color);
macros::make_default_value_getter!(ThemeConfig, primary, Color);
macros::make_default_value_getter!(ThemeConfig, error, Color);

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
    /// The keyboard shortcut that should trigger [action].
    ///
    /// Multiple keys have to be separated by '-', e.g. 'ctrl-a'.
    ///
    /// Important keys:
    /// - 'ctrl', 'shift', 'alt', 'super'
    /// - 'left', 'right', 'up', 'down'
    pub key: String,
    /// The action to be triggered by the keyboard shortcut.
    ///
    /// Must be one of the ones defined in [crate::actions].
    pub action: String,
}

impl KeyBinding {
    pub fn new<T: Action>(key: &str, action: T) -> Self {
        KeyBinding {
            key: key.to_string(),
            action: action.name().to_string(),
        }
    }
}

fn default_key_bindings() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("?", Help),
        KeyBinding::new("l", NextImage),
        KeyBinding::new("h", PreviousImage),
        KeyBinding::new("+", ZoomIn),
        KeyBinding::new("-", ZoomOut),
        KeyBinding::new("left", MoveLeft),
        KeyBinding::new("right", MoveRight),
        KeyBinding::new("up", MoveUp),
        KeyBinding::new("down", MoveDown),
        KeyBinding::new("o", OpenFiles),
        KeyBinding::new("i", ToggleImageInfo),
        KeyBinding::new("q", CloseWindow),
    ]
}

fn read_paths_from_stdin() -> Vec<PathBuf> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut v = Vec::new();
    handle.read_to_end(&mut v).unwrap();

    let input_str = String::from_utf8(v).unwrap();

    input_str.split("\n").map(PathBuf::from).collect()
}

fn read_config_file(config_path_override: Option<PathBuf>) -> anyhow::Result<Config> {
    let mut config: Config = if let Some(config_path) = &config_path_override {
        confy::load_path(config_path)?
    } else {
        confy::load::<Config>(env!("CARGO_PKG_NAME"), Some(CONFIG_FILE_NAME)).unwrap_or_default()
    };

    // setting serde default on `keybindings` doesn't work due to
    // the clap_derive_macro, hence we manually have to override the default here if needed
    if config.keybindings.is_empty() {
        config.keybindings = default_key_bindings();
    }

    if config_path_override.is_none() {
        // store the current config including the default key bindings (if no bindings are specified)
        //
        // this automatically writes the default config
        confy::store(env!("CARGO_PKG_NAME"), CONFIG_FILE_NAME, &config)?;
    }

    Ok(config)
}

/// Parse the CLI arguments and fall back to the config file for all arguments
/// that were not provided.
pub fn parse_cli_args_with_config() -> anyhow::Result<(Vec<PathBuf>, Config)> {
    let mut args = ArgsWithConfig::parse();
    let config = read_config_file(args.config_path)?;

    // parse image paths from stdin if '-' is provided as argument
    if args.images == vec![PathBuf::from("-")] {
        args.images = read_paths_from_stdin();
    }

    // merge with the provided CLI arguments
    let config = config.merge(&mut args.config);

    Ok((args.images, config))
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
