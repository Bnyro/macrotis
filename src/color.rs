use std::{
    fmt::{self, Display},
    str::FromStr,
};

use gpui::Rgba;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            a: 255,
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Color::from_str(v).map_err(|err| de::Error::custom(err))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');

        if s.len() != 6 && s.len() != 8 {
            return Err("Hex color must be axactly 8 characters long, e.g. 00aaccff".to_string());
        }

        let color_binding = s
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .filter_map(|chunk| u8::from_str_radix(&String::from_iter(chunk), 16).ok())
            .collect::<Vec<_>>();

        match color_binding.as_slice() {
            [r, g, b, a] => Ok(Self {
                r: *r,
                g: *g,
                b: *b,
                a: *a,
            }),
            [r, g, b] => Ok(Self {
                r: *r,
                g: *g,
                b: *b,
                a: 255,
            }),
            _ => Err("All chars must be hexadecimal, e.g. 00aaccff".to_string()),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{r:02x}{g:02x}{b:02x}{a:02x}",
            r = self.r,
            g = self.g,
            b = self.b,
            a = self.a
        )
    }
}

impl From<Color> for Rgba {
    fn from(val: Color) -> Self {
        Rgba {
            r: std::convert::Into::<f32>::into(val.r) / 256.0,
            g: std::convert::Into::<f32>::into(val.g) / 256.0,
            b: std::convert::Into::<f32>::into(val.b) / 256.0,
            a: std::convert::Into::<f32>::into(val.a) / 256.0,
        }
    }
}

impl Color {
    pub fn into_rgba(self) -> Rgba {
        self.into()
    }
}
