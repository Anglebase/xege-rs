pub type Color = palette::Srgba<u8>;

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
