use std::{fmt::Display, fs::File, io::BufReader, os::unix::fs::MetadataExt, path::Path};

#[derive(Copy, Clone, Debug)]
pub struct ImageResolution {
    pub width: usize,
    pub height: usize,
}

impl Display for ImageResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{width}x{height}",
            width = self.width,
            height = self.height
        )
    }
}

impl ImageResolution {
    #[allow(clippy::cast_precision_loss)]
    pub fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }
}

pub struct ImageInfo {
    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub file_size: Option<u64>,
    pub resolution: Option<ImageResolution>,
}

impl ImageInfo {
    pub fn from_file_path(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());

        let file_size = File::open(path)
            .ok()
            .and_then(|file| file.metadata().ok())
            .map(|metadata| metadata.size());

        let file_type = File::open(path)
            .ok()
            .and_then(|f| {
                let reader = BufReader::new(f);
                imagesize::reader_type(reader).ok()
            })
            .map(|file_type| format!("{file_type:?}").to_uppercase());

        let resolution = imagesize::size(path)
            .ok()
            .map(|resolution| ImageResolution {
                width: resolution.width,
                height: resolution.height,
            });

        Self {
            file_name,
            file_type,
            file_size,
            resolution,
        }
    }
}
