use std::{ops::Range, path::PathBuf};

use gpui::*;

actions!(app, [Help]);
actions!(app, [NextImage]);
actions!(app, [PrevImage]);
actions!(app, [Quit]);
actions!(app, [OpenFiles]);

pub struct AppWindow {
    focus_handle: FocusHandle,
    image_paths: Vec<PathBuf>,
    selected_img_index: usize,
}

impl Render for AppWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut container = div()
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _cx| window.remove_window())
            .on_action(cx.listener(Self::open_files))
            .on_action(cx.listener(Self::open_help))
            .on_action(cx.listener(Self::next_image))
            .on_action(cx.listener(Self::prev_image))
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .flex();

        if let Some(image) = self.image_paths.get(self.selected_img_index) {
            container = container.child(img(image.clone()));
        }

        container
    }
}

impl AppWindow {
    fn prev_image(&mut self, _action: &PrevImage, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_img_index == 0 {
            return;
        }
        self.selected_img_index -= 1;
        cx.notify();
    }

    fn next_image(&mut self, _action: &NextImage, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_img_index >= self.image_paths.len() - 1 {
            return;
        }

        self.selected_img_index += 1;
        cx.notify();
    }

    fn open_help(&mut self, _action: &Help, _window: &mut Window, cx: &mut Context<Self>) {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(300.0), px(300.0)), cx));

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                kind: WindowKind::Dialog,
                ..Default::default()
            },
            |window, cx| {
                let focus_handle = cx.focus_handle();
                focus_handle.focus(window, cx);

                cx.new(|_cx| HelpWindow { focus_handle })
            },
        );
    }

    fn open_files(&mut self, _action: &OpenFiles, _window: &mut Window, cx: &mut Context<Self>) {
        let recv = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        cx.spawn(async |window, cx| {
            let mut selection_result = recv.await.ok().and_then(|res| res.ok()).and_then(|res| res);

            if let Some(images) = &mut selection_result {
                let _ = window.update(cx, |window, cx| {
                    let prev_size = window.image_paths.len();

                    // append new images and seek to first new image
                    window.image_paths.append(images);
                    window.selected_img_index = prev_size;

                    cx.notify();
                });
            }
        })
        .detach();
    }
}

struct KeyBindingItem {
    key_binding: KeyBinding,
}

impl Render for KeyBindingItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let binding = &self.key_binding;

        div()
            .size_full()
            .flex()
            .p_1()
            .gap_1()
            .items_center()
            .child(
                div().p_1().bg(opaque_grey(0.7, 1.0)).rounded_md().child(
                    binding
                        .keystrokes()
                        .iter()
                        .map(|keystroke| keystroke.key())
                        .collect::<Vec<_>>()
                        .join("-"),
                ),
            )
            .child(binding.action().name())
    }
}

struct HelpWindow {
    focus_handle: FocusHandle,
}

impl Render for HelpWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let keymap = cx.key_bindings();
        let keymap = keymap.borrow();

        div()
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _cx| window.remove_window())
            .bg(white())
            .child(div().child("Help").text_center().text_xl())
            .child(
                uniform_list(
                    "key_bindings",
                    keymap.bindings().count(),
                    cx.processor(|_this, range: Range<usize>, _window, cx| {
                        let keymap = cx.key_bindings();
                        let keymap = keymap.borrow();
                        let key_bindings: Vec<_> = keymap.bindings().collect();

                        (range.start..range.end)
                            .map(|idx| {
                                let key_binding = key_bindings[idx].clone();

                                cx.new(|_| KeyBindingItem { key_binding })
                            })
                            .collect()
                    }),
                )
                .size_full(),
            )
            .size_full()
    }
}

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
                    KeyBinding::new("h", PrevImage, None),
                    KeyBinding::new("o", OpenFiles, None),
                    KeyBinding::new("q", Quit, None),
                ];
                cx.bind_keys(bindings.clone());

                let focus_handle = cx.focus_handle();
                focus_handle.focus(window, cx);

                cx.new(|_| AppWindow {
                    focus_handle,
                    image_paths: paths,
                    selected_img_index: 0,
                })
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
