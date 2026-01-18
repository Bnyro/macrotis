use gpui::*;
use std::path::PathBuf;

use crate::config::CONFIG;

pub struct ZoomableImage {
    image: Option<PathBuf>,
    zoom_factor: f32,
    manual_offset: Point<Pixels>,
}

impl Render for ZoomableImage {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(image) = &self.image {
            let offset = self.calculate_center_offset(window) + self.manual_offset;

            div()
                .id("container")
                .size_full()
                .items_center()
                .justify_center()
                .flex()
                .child(
                    div()
                        .id("image_wrapper")
                        .left(offset.x)
                        .top(offset.y)
                        .child(
                            img(image.clone())
                                .w(self.calculate_width(window))
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
                .text_color(CONFIG.get().unwrap().theme.error.into_rgba())
                .child("No image provided yet")
        }
    }
}

impl ZoomableImage {
    pub fn new(image: Option<PathBuf>) -> Self {
        Self {
            image,
            zoom_factor: 1.0,
            manual_offset: Point::default(),
        }
    }

    pub fn set_image(&mut self, cx: &mut Context<Self>, image: Option<PathBuf>) {
        self.image = image;
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
        self.manual_offset += Point::new(px(0.0), px(-40.0));

        cx.notify();
    }

    pub fn move_down(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(px(0.0), px(40.0));

        cx.notify();
    }

    pub fn move_left(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(px(-40.0), px(0.0));

        cx.notify();
    }

    pub fn move_right(&mut self, cx: &mut Context<Self>) {
        self.manual_offset += Point::new(px(40.0), px(0.0));

        cx.notify();
    }

    fn calculate_width(&self, window: &mut Window) -> Pixels {
        let window_size = window.bounds().size;

        window_size.width / 2.0 * self.zoom_factor
    }

    fn calculate_center_offset(&self, window: &mut Window) -> Point<Pixels> {
        let window_width = window.bounds().size.width;
        let image_width = self.calculate_width(window);

        // image is automatically centered by justify center
        let x = if image_width < window_width {
            Pixels::default()
        } else {
            (window_width - image_width) / 2.0
        };

        // TODO: handle image height
        Point::new(x, Pixels::default())
    }
}
