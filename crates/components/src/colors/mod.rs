use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    const PALLETE: [(&str, i32, i32, i32); 16] = [
        ("black", 0, 0, 0),
        ("dark_blue", 0, 0, 170),
        ("dark_green", 0, 170, 0),
        ("dark_aqua", 0, 170, 170),
        ("dark_red", 170, 0, 0),
        ("dark_purple", 170, 0, 170),
        ("gold", 255, 170, 0),
        ("gray", 170, 170, 170),
        ("dark_gray", 85, 85, 85),
        ("blue", 85, 85, 255),
        ("green", 85, 255, 85),
        ("aqua", 85, 255, 255),
        ("red", 255, 85, 85),
        ("light_purple", 255, 85, 255),
        ("yellow", 255, 255, 85),
        ("white", 255, 255, 255),
    ];

    pub fn to_legacy_name(&self) -> &'static str {
        let mut legacy_color = "white";
        let mut min_dist_sq = i32::MAX;

        let r = self.r as i32;
        let g = self.g as i32;
        let b = self.b as i32;

        for (name, palette_r, palette_g, palette_b) in Color::PALLETE {
            let dr = r - palette_r;
            let dg = g - palette_g;
            let db = b - palette_b;
            let dist_sq = dr * dr + dg * dg + db * db;

            if dist_sq < min_dist_sq {
                legacy_color = name;
                min_dist_sq = dist_sq;
                if dist_sq == 0 {
                    break;
                }
            }
        }

        legacy_color
    }

    const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn from(hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b) = if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| "invalid red")?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| "invalid green")?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| "invalid blue")?;
            (r, g, b)
        } else if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid red")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid green")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid blue")?;
            (r, g, b)
        } else {
            let color = Color::from_name(hex);
            if color.is_some() {
                return Ok(color.unwrap());
            }
            return Err(format!(
                "invalid hex color, expected 3 or 6 characters ({})",
                hex
            ));
        };
        Ok(Color { r, g, b })
    }

    pub fn to_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn lerp(&self, other: &Self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        let lerp_up = |start: u8, end: u8| -> u8 {
            (start as f32 + (end as f32 - start as f32) * factor).round() as u8
        };
        Color {
            r: lerp_up(self.r, other.r),
            g: lerp_up(self.g, other.g),
            b: lerp_up(self.b, other.b),
        }
    }

    pub fn from_name(name: &str) -> Option<Color> {
        match name {
            "black" => Some(Color::BLACK),
            "dark_blue" => Some(Color::DARK_BLUE),
            "dark_green" => Some(Color::DARK_GREEN),
            "dark_aqua" => Some(Color::DARK_AQUA),
            "dark_red" => Some(Color::DARK_RED),
            "dark_purple" => Some(Color::DARK_PURPLE),
            "gold" => Some(Color::GOLD),
            "gray" => Some(Color::GRAY),
            "dark_gray" => Some(Color::DARK_GRAY),
            "blue" => Some(Color::BLUE),
            "green" => Some(Color::GREEN),
            "aqua" => Some(Color::AQUA),
            "red" => Some(Color::RED),
            "light_purple" => Some(Color::LIGHT_PURPLE),
            "yellow" => Some(Color::YELLOW),
            "white" => Some(Color::WHITE),
            _ => None,
        }
    }

    pub fn from_legacy_char(c: char) -> Option<Color> {
        match c {
            '0' => Some(Color::BLACK),
            '1' => Some(Color::DARK_BLUE),
            '2' => Some(Color::DARK_GREEN),
            '3' => Some(Color::DARK_AQUA),
            '4' => Some(Color::DARK_RED),
            '5' => Some(Color::DARK_PURPLE),
            '6' => Some(Color::GOLD),
            '7' => Some(Color::GRAY),
            '8' => Some(Color::DARK_GRAY),
            '9' => Some(Color::BLUE),
            'a' => Some(Color::GREEN),
            'b' => Some(Color::AQUA),
            'c' => Some(Color::RED),
            'd' => Some(Color::LIGHT_PURPLE),
            'e' => Some(Color::YELLOW),
            'f' => Some(Color::WHITE),
            _ => None,
        }
    }
}

impl Color {
    pub const BLACK: Color = Color::new(0x00, 0x00, 0x00); // &0
    pub const DARK_BLUE: Color = Color::new(0x00, 0x00, 0xAA); // &1
    pub const DARK_GREEN: Color = Color::new(0x00, 0xAA, 0x00); // &2
    pub const DARK_AQUA: Color = Color::new(0x00, 0xAA, 0xAA); // &3
    pub const DARK_RED: Color = Color::new(0xAA, 0x00, 0x00); // &4
    pub const DARK_PURPLE: Color = Color::new(0xAA, 0x00, 0xAA); // &5
    pub const GOLD: Color = Color::new(0xFF, 0xAA, 0x00); // &6
    pub const GRAY: Color = Color::new(0xAA, 0xAA, 0xAA); // &7
    pub const DARK_GRAY: Color = Color::new(0x55, 0x55, 0x55); // &8
    pub const BLUE: Color = Color::new(0x55, 0x55, 0xFF); // &9
    pub const GREEN: Color = Color::new(0x55, 0xFF, 0x55); // &a
    pub const AQUA: Color = Color::new(0x55, 0xFF, 0xFF); // &b
    pub const RED: Color = Color::new(0xFF, 0x55, 0x55); // &c
    pub const LIGHT_PURPLE: Color = Color::new(0xFF, 0x55, 0xFF); // &d
    pub const YELLOW: Color = Color::new(0xFF, 0xFF, 0x55); // &e
    pub const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF); // &f
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(&self.to_hex_string());
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(color) = Color::from_name(s) {
            return Ok(color);
        }

        if s.starts_with("#") && (s.len() == 7 || s.len() == 4) {
            return Color::from(s);
        }

        Err(format!("unknown color: {}", s))
    }
}
