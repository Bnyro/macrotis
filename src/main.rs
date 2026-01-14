use crate::{actions::*, windows::main_window::AppWindow};
use gpui::*;
use std::path::PathBuf;

mod actions;
mod widgets;
mod windows;

fn main() {
    let app = Application::new();
    let args: Vec<_> = std::env::args().skip(1).collect();
    let cwd = std::env::current_dir().unwrap_or_default();

    let paths: Vec<_> = args
        .iter()
        .map(|arg| {
            let mut path = PathBuf::from(arg.as_str());

            if path.is_relative() {
                path = cwd.join(path);
            }

            path
        })
        .filter(|path| path.is_file())
        .collect();

    app.run(move |app| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(500.0), px(300.0)), app));

        let window_opts = WindowOptions {
            window_background: WindowBackgroundAppearance::Transparent,
            window_decorations: Some(WindowDecorations::Client),
            window_bounds: Some(window_bounds),
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
                    KeyBinding::new("q", Quit, None),
                ];
                cx.bind_keys(bindings.clone());

                cx.new(|cx| AppWindow::new(window, cx, paths))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
