use std::path::PathBuf;

use crate::{
    actions::*,
    config::CONFIG,
    widgets::{image_info::ImageInfoWidget, zoomable_image::ZoomableImage},
    windows::help_window::HelpWindow,
};
use gpui::{prelude::FluentBuilder, *};

pub struct AppWindow {
    focus_handle: FocusHandle,
    image_paths: Vec<PathBuf>,
    selected_img_index: usize,
    zoomable_image: Entity<ZoomableImage>,
    show_image_info: bool,
}

impl Render for AppWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(if CONFIG.get().unwrap().no_transparency {
                CONFIG.get().unwrap().theme.background.into_rgba()
            } else {
                transparent_black().to_rgb()
            })
            .text_color(CONFIG.get().unwrap().theme.foreground.into_rgba())
            .track_focus(&self.focus_handle)
            .on_action(|_: &CloseWindow, window, _cx| window.remove_window())
            .on_action(cx.listener(Self::open_files))
            .on_action(cx.listener(Self::open_help))
            .on_action(cx.listener(Self::toggle_fullscreen))
            .on_action(cx.listener(Self::next_image))
            .on_action(cx.listener(Self::prev_image))
            .on_action(cx.listener(Self::first_image))
            .on_action(cx.listener(Self::last_image))
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .on_action(cx.listener(Self::move_left))
            .on_action(cx.listener(Self::move_right))
            .on_action(cx.listener(Self::toggle_image_info))
            .size_full()
            .relative()
            .child(
                div()
                    .size_full()
                    .absolute()
                    .child(self.zoomable_image.clone()),
            )
            .when_some(
                self.selected_image().take_if(|_| self.show_image_info),
                |container, image_path| {
                    container.child(
                        div()
                            .absolute()
                            .top_2()
                            .right_2()
                            .child(cx.new(|_| ImageInfoWidget::new(image_path))),
                    )
                },
            )
            .when_some(self.selected_image(), |container, _| {
                container.child(div().absolute().bottom_2().right_2().child(format!(
                    "{}/{}",
                    self.selected_img_index + 1,
                    self.image_paths.len()
                )))
            })
    }
}

impl AppWindow {
    pub fn new(window: &mut Window, cx: &mut App, image_paths: Vec<PathBuf>) -> Self {
        window.set_window_title(env!("CARGO_PKG_NAME"));

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let image = image_paths.first().cloned();
        Self {
            focus_handle,
            image_paths,
            selected_img_index: 0,
            zoomable_image: cx.new(|_| ZoomableImage::new(image)),
            show_image_info: true,
        }
    }

    fn selected_image(&self) -> Option<PathBuf> {
        self.image_paths.get(self.selected_img_index).cloned()
    }

    fn prev_image(
        &mut self,
        _action: &PreviousImage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_img_index == 0 {
            return;
        }

        self.selected_img_index -= 1;
        self.set_image(cx, self.selected_image());
    }

    fn next_image(&mut self, _action: &NextImage, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_img_index + 1 >= self.image_paths.len() {
            return;
        }

        self.selected_img_index += 1;
        self.set_image(cx, self.selected_image());
    }

    fn first_image(
        &mut self,
        _action: &GotoFirstImage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.image_paths.is_empty() {
            return;
        }

        self.selected_img_index = 0;
        self.set_image(cx, self.selected_image());
    }

    fn last_image(
        &mut self,
        _action: &GotoLastImage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.image_paths.is_empty() {
            return;
        }

        self.selected_img_index = self.image_paths.len() - 1;
        self.set_image(cx, self.selected_image());
    }

    fn set_image(&mut self, cx: &mut Context<Self>, image: Option<PathBuf>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.set_image(cx, image);
        });
    }

    #[allow(clippy::unused_self)]
    fn open_help(&mut self, _action: &Help, _window: &mut Window, cx: &mut Context<Self>) {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(300.0), px(300.0)), cx));

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                kind: WindowKind::Dialog,
                ..Default::default()
            },
            |window, cx| cx.new(|cx| HelpWindow::new(window, cx)),
        );
    }

    #[allow(clippy::unused_self)]
    fn toggle_fullscreen(
        &mut self,
        _action: &ToggleFullscreen,
        window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        window.toggle_fullscreen();
    }

    #[allow(clippy::unused_self)]
    fn open_files(&mut self, _action: &OpenFiles, _window: &mut Window, cx: &mut Context<Self>) {
        let recv = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: None,
        });

        cx.spawn(async |window, cx| {
            let mut selection_result = recv.await.ok().and_then(std::result::Result::ok).and_then(|res| res);

            if let Some(images) = &mut selection_result {
                let _ = window.update(cx, |this, cx| {
                    let prev_size = this.image_paths.len();

                    // append new images and seek to first new image
                    this.image_paths.append(images);
                    this.selected_img_index = prev_size;

                    this.set_image(cx, this.selected_image());
                });
            }
        })
        .detach();
    }

    pub fn zoom_in(&mut self, _action: &ZoomIn, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.zoom_in(cx);
        });
    }

    pub fn zoom_out(&mut self, _action: &ZoomOut, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.zoom_out(cx);
        });
    }

    pub fn move_left(&mut self, _action: &MoveLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_left(cx);
        });
    }

    pub fn move_right(
        &mut self,
        _action: &MoveRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_right(cx);
        });
    }

    pub fn move_up(&mut self, _action: &MoveUp, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_up(cx);
        });
    }

    pub fn move_down(&mut self, _action: &MoveDown, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_down(cx);
        });
    }

    pub fn toggle_image_info(
        &mut self,
        _action: &ToggleImageInfo,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_image_info = !self.show_image_info;

        cx.notify();
    }
}
