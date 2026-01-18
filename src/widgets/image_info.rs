use std::path::PathBuf;

use bytesize::ByteSize;
use gpui::{prelude::FluentBuilder, *};

use crate::image_info::ImageInfo;

pub struct ImageInfoWidget {
    path: PathBuf,
}

impl ImageInfoWidget {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Render for ImageInfoWidget {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let img_info = ImageInfo::from_file_path(&self.path);

        div()
            .border(px(2.0))
            .border_color(black())
            .rounded_md()
            .px_2()
            .bg(opaque_grey(0.3, 0.5))
            .text_color(white())
            .flex_col()
            .when_some(img_info.file_name, |div, file_name| {
                div.child(format!("Filename: {file_name}"))
            })
            .when_some(img_info.file_type, |div, file_type| {
                div.child(format!("Type: {file_type}"))
            })
            .when_some(img_info.resolution, |div, resolution| {
                div.child(format!("Resolution: {resolution}"))
            })
            .when_some(img_info.file_size, |div, file_size| {
                div.child(format!("Size: {}", ByteSize::b(file_size)))
            })
    }
}
