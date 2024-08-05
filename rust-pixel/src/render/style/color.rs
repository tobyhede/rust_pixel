// RustPixel
// copyright zipxing@hotmail.com 2022~2024

//! Defines styles color

use crate::render::style::ColorPro;
#[cfg(not(any(feature = "sdl", target_os = "android", target_os = "ios", target_arch = "wasm32")))]
use crossterm::style::Color as CColor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgba(u8, u8, u8, u8),
    Indexed(u8),
    Professional(ColorPro),
}

impl Color {
    pub fn get_rgba(self) -> (u8, u8, u8, u8) {
        let cidx: usize;
        match self {
            Color::Reset => cidx = 8,
            Color::Black => cidx = 0,
            Color::Red => cidx = 1,
            Color::Green => cidx = 2,
            Color::Yellow => cidx = 3,
            Color::Blue => cidx = 4,
            Color::Magenta => cidx = 5,
            Color::Cyan => cidx = 6,
            Color::Gray => cidx = 7,
            Color::DarkGray => cidx = 8,
            Color::LightRed => cidx = 9,
            Color::LightGreen => cidx = 10,
            Color::LightYellow => cidx = 11,
            Color::LightBlue => cidx = 12,
            Color::LightMagenta => cidx = 13,
            Color::LightCyan => cidx = 14,
            Color::White => cidx = 15,
            Color::Indexed(i) => cidx = i as usize,
            Color::Rgba(r, g, b, a) => return (r, g, b, a),
            Color::Professional(cpro) => return cpro.get_srgba_u8(),
        };
        (
            ANSI_COLOR_RGB[cidx][0],
            ANSI_COLOR_RGB[cidx][1],
            ANSI_COLOR_RGB[cidx][2],
            255,
        )
    }
}

#[cfg(not(any(feature = "sdl", target_os = "android", target_os = "ios", target_arch = "wasm32")))]
impl From<Color> for CColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => CColor::Reset,
            Color::Black => CColor::Black,
            Color::Red => CColor::DarkRed,
            Color::Green => CColor::DarkGreen,
            Color::Yellow => CColor::DarkYellow,
            Color::Blue => CColor::DarkBlue,
            Color::Magenta => CColor::DarkMagenta,
            Color::Cyan => CColor::DarkCyan,
            Color::Gray => CColor::Grey,
            Color::DarkGray => CColor::DarkGrey,
            Color::LightRed => CColor::Red,
            Color::LightGreen => CColor::Green,
            Color::LightBlue => CColor::Blue,
            Color::LightYellow => CColor::Yellow,
            Color::LightMagenta => CColor::Magenta,
            Color::LightCyan => CColor::Cyan,
            Color::White => CColor::White,
            Color::Indexed(i) => CColor::AnsiValue(i),
            Color::Rgba(r, g, b, _a) => CColor::Rgb { r, g, b },
            Color::Professional(cpro) => {
                let (r, g, b, _a) = cpro.get_srgba_u8();
                CColor::Rgb { r, g, b }
            }
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => 0,
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::Gray => 7,
            Color::DarkGray => 8,
            Color::LightRed => 9,
            Color::LightGreen => 10,
            Color::LightBlue => 11,
            Color::LightYellow => 12,
            Color::LightMagenta => 13,
            Color::LightCyan => 14,
            Color::White => 15,
            Color::Indexed(i) => i,
            Color::Rgba(_r, _g, _b, _a) => 0,
            Color::Professional(_cpro) => 0,
        }
    }
}

pub const ANSI_COLOR_RGB: [[u8; 3]; 256] = [
    [0, 0, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 128, 0],
    [0, 0, 128],
    [128, 0, 128],
    [0, 128, 128],
    [192, 192, 192],
    [128, 128, 128],
    [255, 0, 0],
    [0, 255, 0],
    [255, 255, 0],
    [0, 0, 255],
    [255, 0, 255],
    [0, 255, 255],
    [255, 255, 255],
    [0, 0, 0],
    [0, 0, 95],
    [0, 0, 135],
    [0, 0, 175],
    [0, 0, 215],
    [0, 0, 255],
    [0, 95, 0],
    [0, 95, 95],
    [0, 95, 135],
    [0, 95, 175],
    [0, 95, 215],
    [0, 95, 255],
    [0, 135, 0],
    [0, 135, 95],
    [0, 135, 135],
    [0, 135, 175],
    [0, 135, 215],
    [0, 135, 255],
    [0, 175, 0],
    [0, 175, 95],
    [0, 175, 135],
    [0, 175, 175],
    [0, 175, 215],
    [0, 175, 255],
    [0, 215, 0],
    [0, 215, 95],
    [0, 215, 135],
    [0, 215, 175],
    [0, 215, 215],
    [0, 215, 255],
    [0, 255, 0],
    [0, 255, 95],
    [0, 255, 135],
    [0, 255, 175],
    [0, 255, 215],
    [0, 255, 255],
    [95, 0, 0],
    [95, 0, 95],
    [95, 0, 135],
    [95, 0, 175],
    [95, 0, 215],
    [95, 0, 255],
    [95, 95, 0],
    [95, 95, 95],
    [95, 95, 135],
    [95, 95, 175],
    [95, 95, 215],
    [95, 95, 255],
    [95, 135, 0],
    [95, 135, 95],
    [95, 135, 135],
    [95, 135, 175],
    [95, 135, 215],
    [95, 135, 255],
    [95, 175, 0],
    [95, 175, 95],
    [95, 175, 135],
    [95, 175, 175],
    [95, 175, 215],
    [95, 175, 255],
    [95, 215, 0],
    [95, 215, 95],
    [95, 215, 135],
    [95, 215, 175],
    [95, 215, 215],
    [95, 215, 255],
    [95, 255, 0],
    [95, 255, 95],
    [95, 255, 135],
    [95, 255, 175],
    [95, 255, 215],
    [95, 255, 255],
    [135, 0, 0],
    [135, 0, 95],
    [135, 0, 135],
    [135, 0, 175],
    [135, 0, 215],
    [135, 0, 255],
    [135, 95, 0],
    [135, 95, 95],
    [135, 95, 135],
    [135, 95, 175],
    [135, 95, 215],
    [135, 95, 255],
    [135, 135, 0],
    [135, 135, 95],
    [135, 135, 135],
    [135, 135, 175],
    [135, 135, 215],
    [135, 135, 255],
    [135, 175, 0],
    [135, 175, 95],
    [135, 175, 135],
    [135, 175, 175],
    [135, 175, 215],
    [135, 175, 255],
    [135, 215, 0],
    [135, 215, 95],
    [135, 215, 135],
    [135, 215, 175],
    [135, 215, 215],
    [135, 215, 255],
    [135, 255, 0],
    [135, 255, 95],
    [135, 255, 135],
    [135, 255, 175],
    [135, 255, 215],
    [135, 255, 255],
    [175, 0, 0],
    [175, 0, 95],
    [175, 0, 135],
    [175, 0, 175],
    [175, 0, 215],
    [175, 0, 255],
    [175, 95, 0],
    [175, 95, 95],
    [175, 95, 135],
    [175, 95, 175],
    [175, 95, 215],
    [175, 95, 255],
    [175, 135, 0],
    [175, 135, 95],
    [175, 135, 135],
    [175, 135, 175],
    [175, 135, 215],
    [175, 135, 255],
    [175, 175, 0],
    [175, 175, 95],
    [175, 175, 135],
    [175, 175, 175],
    [175, 175, 215],
    [175, 175, 255],
    [175, 215, 0],
    [175, 215, 95],
    [175, 215, 135],
    [175, 215, 175],
    [175, 215, 215],
    [175, 215, 255],
    [175, 255, 0],
    [175, 255, 95],
    [175, 255, 135],
    [175, 255, 175],
    [175, 255, 215],
    [175, 255, 255],
    [215, 0, 0],
    [215, 0, 95],
    [215, 0, 135],
    [215, 0, 175],
    [215, 0, 215],
    [215, 0, 255],
    [215, 95, 0],
    [215, 95, 95],
    [215, 95, 135],
    [215, 95, 175],
    [215, 95, 215],
    [215, 95, 255],
    [215, 135, 0],
    [215, 135, 95],
    [215, 135, 135],
    [215, 135, 175],
    [215, 135, 215],
    [215, 135, 255],
    [215, 175, 0],
    [215, 175, 95],
    [215, 175, 135],
    [215, 175, 175],
    [215, 175, 215],
    [215, 175, 255],
    [215, 215, 0],
    [215, 215, 95],
    [215, 215, 135],
    [215, 215, 175],
    [215, 215, 215],
    [215, 215, 255],
    [215, 255, 0],
    [215, 255, 95],
    [215, 255, 135],
    [215, 255, 175],
    [215, 255, 215],
    [215, 255, 255],
    [255, 0, 0],
    [255, 0, 95],
    [255, 0, 135],
    [255, 0, 175],
    [255, 0, 215],
    [255, 0, 255],
    [255, 95, 0],
    [255, 95, 95],
    [255, 95, 135],
    [255, 95, 175],
    [255, 95, 215],
    [255, 95, 255],
    [255, 135, 0],
    [255, 135, 95],
    [255, 135, 135],
    [255, 135, 175],
    [255, 135, 215],
    [255, 135, 255],
    [255, 175, 0],
    [255, 175, 95],
    [255, 175, 135],
    [255, 175, 175],
    [255, 175, 215],
    [255, 175, 255],
    [255, 215, 0],
    [255, 215, 95],
    [255, 215, 135],
    [255, 215, 175],
    [255, 215, 215],
    [255, 215, 255],
    [255, 255, 0],
    [255, 255, 95],
    [255, 255, 135],
    [255, 255, 175],
    [255, 255, 215],
    [255, 255, 255],
    [8, 8, 8],
    [18, 18, 18],
    [28, 28, 28],
    [38, 38, 38],
    [48, 48, 48],
    [58, 58, 58],
    [68, 68, 68],
    [78, 78, 78],
    [88, 88, 88],
    [98, 98, 98],
    [108, 108, 108],
    [118, 118, 118],
    [128, 128, 128],
    [138, 138, 138],
    [148, 148, 148],
    [158, 158, 158],
    [168, 168, 168],
    [178, 178, 178],
    [188, 188, 188],
    [198, 198, 198],
    [208, 208, 208],
    [218, 218, 218],
    [228, 228, 228],
    [238, 238, 238],
];
