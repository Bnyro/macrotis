use std::{
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
    sync::OnceLock,
};

use clap::{ArgAction, Parser};
use clap_serde_derive::ClapSerde;
use serde::{Deserialize, Serialize};

use crate::color::Color;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

/// Display images from files.
///
/// Example usage: `macrotis example1.png example2.svg`
#[derive(Parser)]
#[clap(author, version, about)]
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
    #[arg(short, long, action = ArgAction::SetFalse)]
    pub transparent: bool,
    /// Theme config.
    #[command(flatten)]
    pub theme: ThemeConfig,
}

#[derive(Serialize, Deserialize, Debug, clap::Args)]
pub struct ThemeConfig {
    /// Background color.
    #[arg(short, long, default_value = "#1e1e2e")]
    pub background: Color,
    /// Text color.
    #[arg(short, long, default_value = "#cdd6f4")]
    pub foreground: Color,

    /// Surface color.
    #[arg(short, long, default_value = "#6c708688")]
    pub surface: Color,
    /// Primary color.
    #[arg(short, long, default_value = "#cba6f7")]
    pub primary: Color,

    /// Error color.
    #[arg(short, long, default_value = "#d20f39")]
    pub error: Color,
}

// duplicated defaults implementation, i.e. default theme is redefined :/
// blocked by https://github.com/clap-rs/clap/issues/3116
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

fn read_paths_from_stdin() -> Vec<PathBuf> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut v = Vec::new();
    handle.read_to_end(&mut v).unwrap();

    let input_str = String::from_utf8(v).unwrap();

    input_str.split("\n").map(PathBuf::from).collect()
}

pub fn parse_config() -> anyhow::Result<(Vec<PathBuf>, Config)> {
    let mut args = ArgsWithConfig::parse();

    // parse image paths from stdin if '-' is provided as argument
    if args.images == vec![PathBuf::from("-")] {
        args.images = read_paths_from_stdin();
    }

    let config: Config = if let Some(config_path) = args.config_path {
        confy::load_path(config_path)?
    } else {
        confy::load::<Config>(env!("CARGO_PKG_NAME"), Some(env!("CARGO_PKG_NAME")))
            .unwrap_or_default()
    };

    let config = config.merge(&mut args.config);

    Ok((args.images, config))
}
