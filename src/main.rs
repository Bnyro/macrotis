use crate::{
    actions::*,
    config::{CONFIG, parse_config},
    windows::main_window::AppWindow,
};
use gpui::*;
use std::process::exit;

mod actions;
mod color;
mod config;
mod image_info;
mod macros;
mod widgets;
mod windows;

fn main() {
    let app = Application::new();

    let (paths, config) = match parse_config() {
        Ok((paths, config)) => (paths, config),
        Err(err) => {
            eprintln!("Failed to parse config: {err}");
            exit(1);
        }
    };
    CONFIG.set(config).unwrap();

    let cwd = std::env::current_dir().unwrap_or_default();
    let paths: Vec<_> = paths
        .iter()
        .map(|path| {
            if path.is_relative() {
                cwd.join(path)
            } else {
                path.to_path_buf()
            }
        })
        .filter(|path| path.is_file())
        .collect();

    app.run(move |app| {
        let window_opts = WindowOptions {
            window_background: WindowBackgroundAppearance::Transparent,
            window_decorations: Some(WindowDecorations::Client),
            ..Default::default()
        };
        app.spawn(async move |cx| {
            cx.open_window(window_opts, |window, cx| {
                let bindings = [
                    KeyBinding::new("?", Help, None),
                    KeyBinding::new("l", NextImage, None),
                    KeyBinding::new("h", PreviousImage, None),
                    KeyBinding::new("+", ZoomIn, None),
                    KeyBinding::new("-", ZoomOut, None),
                    KeyBinding::new("left", MoveLeft, None),
                    KeyBinding::new("right", MoveRight, None),
                    KeyBinding::new("up", MoveUp, None),
                    KeyBinding::new("down", MoveDown, None),
                    KeyBinding::new("o", OpenFiles, None),
                    KeyBinding::new("i", ToggleImageInfo, None),
                    KeyBinding::new("q", CloseWindow, None),
                ];
                cx.bind_keys(bindings.clone());

                cx.new(|cx| AppWindow::new(window, cx, paths))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
