/// Color
pub struct Color(palette::Srgba<u8>);

impl Color {
    pub fn from_argb(argb: u32) -> Self {
        Color(palette::Srgba::<u8>::new(
            ((argb >> 16) & 0xff) as _,
            ((argb >> 8) & 0xff) as _,
            ((argb) & 0xff) as _,
            ((argb >> 24) & 0xff) as _,
        ))
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color(palette::Srgba::<u8>::new(red, green, blue, alpha))
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba(red, green, blue, 255)
    }

    pub fn gray(gray: u8) -> Self {
        Self::rgb(gray, gray, gray)
    }

    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::rgba(red, green, blue, alpha)
    }
}

/// This trait is the interface between other color formats and the EGE used color format(`u32`, ARGB).
pub trait IntoARGB {
    fn into_argb(&self) -> u32;
}

impl IntoARGB for palette::Srgba<u8> {
    fn into_argb(&self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
    }
}

impl IntoARGB for Color {
    fn into_argb(&self) -> u32 {
        self.0.into_argb()
    }
}

impl IntoARGB for palette::rgb::Srgb<u8> {
    fn into_argb(&self) -> u32 {
        ((255u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
    }
}

// re-exporting palette's named colors
pub use palette::named::*;

use std::ops::{Deref, DerefMut};

impl Deref for Color {
    type Target = palette::Srgba<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
