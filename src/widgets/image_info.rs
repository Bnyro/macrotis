use std::path::PathBuf;

use bytesize::ByteSize;
use gpui::{prelude::FluentBuilder, *};

use crate::{config::CONFIG, image_info::ImageInfo};

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
            .border_color(CONFIG.get().unwrap().theme.primary.into_rgba())
            .rounded_md()
            .px_2()
            .bg(CONFIG.get().unwrap().theme.surface.into_rgba())
            .text_color(CONFIG.get().unwrap().theme.foreground.into_rgba())
            .flex_col()
            .when_some(img_info.file_name, |div, file_name| {
                div.child(t!("image.filename", "filename" => file_name).to_string())
            })
            .when_some(img_info.file_type, |div, file_type| {
                div.child(t!("image.type", "type" => file_type).to_string())
            })
            .when_some(img_info.resolution, |div, resolution| {
                div.child(t!("image.resolution", "resolution" => resolution).to_string())
            })
            .when_some(img_info.file_size, |div, file_size| {
                div.child(t!("image.size", "size" => ByteSize::b(file_size)).to_string())
            })
    }
}
