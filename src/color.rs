/// Color
pub type Color = palette::Srgba<u8>;

/// This trait is the interface between other color formats and the EGE used color format(`u32`, ARGB).
pub trait IntoARGB {
    fn into_argb(&self) -> u32;
}

impl IntoARGB for Color {
    fn into_argb(&self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
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
