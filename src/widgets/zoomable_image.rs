use gpui::*;
use std::path::PathBuf;

use crate::{
    config::CONFIG,
    image_info::{self, ImageInfo},
};

pub struct ZoomableImage {
    image: Option<(PathBuf, ImageInfo)>,
    zoom_factor: f32,
    manual_offset: Point<Pixels>,
    move_offset_px: Pixels,
}

impl Render for ZoomableImage {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(image) = &self.image {
            div()
                .id("container")
                .size_full()
                .items_center()
                .justify_center()
                .flex()
                .overflow_scroll()
                .child(
                    div()
                        .id("image_wrapper")
                        .left(self.manual_offset.x * self.zoom_factor)
                        .top(self.manual_offset.y * self.zoom_factor)
                        .child(
                            img(image.0.clone())
                                .w(self.calculate_image_size(window).width)
                                .with_fallback(|| {
                                    div()
                                        .text_color(CONFIG.get().unwrap().theme.error.into_rgba())
                                        .child("failed to load image")
                                        .into_any_element()
                                }),
                        ),
                )
        } else {
            div()
                .id("container")
                .size_full()
                .flex()
                .justify_center()
                .items_center()
                .text_color(CONFIG.get().unwrap().theme.error.into_rgba())
                .child("No image provided yet")
        }
    }
}

impl ZoomableImage {
    pub fn new(image_path: Option<PathBuf>) -> Self {
        Self {
            image: image_path.map(|image_path| {
                let info = image_info::ImageInfo::from_file_path(&image_path);

                (image_path, info)
            }),
            zoom_factor: 1.0,
            manual_offset: Point::default(),
            move_offset_px: px(40.0),
        }
    }

    pub fn set_image(&mut self, cx: &mut Context<Self>, image_path: Option<PathBuf>) {
        self.image = image_path.map(|image_path| {
            let info = image_info::ImageInfo::from_file_path(&image_path);

            (image_path, info)
        });

        cx.notify();
    }

    pub fn zoom_in(&mut self, cx: &mut Context<Self>) {
        self.zoom_factor *= 1.2;

        cx.notify();
    }

    pub fn zoom_out(&mut self, cx: &mut Context<Self>) {
        self.zoom_factor *= 0.8;

        cx.notify();
    }

    pub fn move_up(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(px(0.0), self.move_offset_px);

        cx.notify();
    }

    pub fn move_down(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(px(0.0), -self.move_offset_px);

        cx.notify();
    }

    pub fn move_left(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(self.move_offset_px, px(0.0));

        cx.notify();
    }

    pub fn move_right(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(-self.move_offset_px, px(0.0));

        cx.notify();
    }

    fn calculate_image_size(&self, window: &mut Window) -> Size<Pixels> {
        let window_size = window.bounds().size;
        let width = window_size.width / 2.0 * self.zoom_factor;

        if let Some(image_resolution) = self.image.as_ref().and_then(|(_, info)| info.resolution) {
            let height = width / image_resolution.aspect_ratio();
            size(width, height)
        } else {
            size(width, Pixels::default())
        }
    }
}
