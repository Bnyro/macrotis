use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;

use clap::Parser;
use clap_serde_derive::ClapSerde;
use gpui::Action;
use gpui::Rgba;

use super::Color;
use crate::actions::*;
use crate::config::ArgsWithConfig;
use crate::config::Config;
use crate::config::KeyBinding;

pub static CONFIG: OnceLock<Config> = OnceLock::new();
static CONFIG_FILE_NAME: &str = "config";

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
        KeyBinding::new("f", ToggleFullscreen),
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

    // HACK: setting serde default on `keybindings` doesn't work due to
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

impl From<Color> for Rgba {
    fn from(val: Color) -> Self {
        Rgba {
            r: std::convert::Into::<f32>::into(val.r) / 256.0,
            g: std::convert::Into::<f32>::into(val.g) / 256.0,
            b: std::convert::Into::<f32>::into(val.b) / 256.0,
            a: std::convert::Into::<f32>::into(val.a) / 256.0,
        }
    }
}

impl Color {
    pub fn into_rgba(self) -> Rgba {
        self.into()
    }
}
