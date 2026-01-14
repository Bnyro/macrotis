use std::path::PathBuf;

use crate::{actions::*, widgets::zoomable_image::ZoomableImage, windows::help_window::HelpWindow};
use gpui::*;

pub struct AppWindow {
    pub focus_handle: FocusHandle,
    pub image_paths: Vec<PathBuf>,
    pub selected_img_index: usize,
    pub zoomable_image: Entity<ZoomableImage>,
}

impl Render for AppWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _cx| window.remove_window())
            .on_action(cx.listener(Self::open_files))
            .on_action(cx.listener(Self::open_help))
            .on_action(cx.listener(Self::next_image))
            .on_action(cx.listener(Self::prev_image))
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .on_action(cx.listener(Self::move_left))
            .on_action(cx.listener(Self::move_right))
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .flex()
            .child(self.zoomable_image.clone())
    }
}

impl AppWindow {
    pub fn new(window: &mut Window, cx: &mut App, image_paths: Vec<PathBuf>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        let image = image_paths.first().cloned();
        Self {
            focus_handle,
            image_paths,
            selected_img_index: 0,
            zoomable_image: cx.new(|_| ZoomableImage::new(image)),
        }
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

    pub fn zoom_in(&mut self, _action: &ZoomIn, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.zoom_in(cx);
        })
    }

    pub fn zoom_out(&mut self, _action: &ZoomOut, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.zoom_out(cx);
        })
    }

    pub fn move_left(&mut self, _action: &MoveLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_left(cx);
        })
    }

    pub fn move_right(
        &mut self,
        _action: &MoveRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_right(cx);
        })
    }

    pub fn move_up(&mut self, _action: &MoveUp, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_up(cx);
        })
    }

    pub fn move_down(&mut self, _action: &MoveDown, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoomable_image.update(cx, |zoomable_image, cx| {
            zoomable_image.move_down(cx);
        })
    }
}
