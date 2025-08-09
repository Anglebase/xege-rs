use mats::Mat;
use std::fmt::Display;
use xege_ffi::*;

use crate::{DrawableDevice, GraphicsEnvironment, ImageDraw};

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Memory allocation failed during read operation.")]
    AllocError,
    #[error("The file does not exist.")]
    FileNotFound,
    #[error("Pointer conversion failed.")]
    NullPointer,
    #[error("Reading failed.")]
    IOError,
    #[error("Other Unknown error.")]
    UnknownError,
    #[error("Path parsing error.")]
    PathParserError,
}

/// Image
#[derive(Debug)]
pub struct Image {
    ptr: *mut ege_IMAGE,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image({:p})", self.ptr)
    }
}

impl DrawableDevice for Image {
    fn const_ptr(&self) -> *const ege_IMAGE {
        self.ptr as *const _
    }

    fn mut_ptr(&mut self) -> *mut ege_IMAGE {
        self.ptr
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ege_delimage(self.ptr) };
    }
}

impl Image {
    /// Create a new image.
    ///
    /// # Parameters
    /// - `width`: The width of the image.
    /// - `height`: The height of the image.
    ///
    /// # Returns
    /// A new `Image` object.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            ptr: unsafe { ege_newimage1(width.max(1) as _, height.max(1) as _) },
        }
    }

    pub(crate) fn handle_result(result: i32) -> Result<(), ImageError> {
        match result {
            xege_ffi::ege_graphics_errors_grOk => Ok(()),
            xege_ffi::ege_graphics_errors_grFileNotFound => Err(ImageError::FileNotFound),
            xege_ffi::ege_graphics_errors_grAllocError => Err(ImageError::AllocError),
            xege_ffi::ege_graphics_errors_grNullPointer => Err(ImageError::NullPointer),
            xege_ffi::ege_graphics_errors_grIOerror => Err(ImageError::IOError),
            _ => Err(ImageError::UnknownError),
        }
    }

    /// Load an image from a file.
    ///
    /// # Parameters
    /// - `filename`: The filename of the image.
    ///
    /// # Returns
    /// A new `Image` object. Or an error.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ImageError> {
        let ptr = unsafe { ege_newimage() };
        let path = path
            .as_ref()
            .to_str()
            .map_or_else(|| Err(ImageError::PathParserError), |s| Ok(s))?
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let result = unsafe { ege_getimage3(ptr, path.as_ptr(), 0, 0) };
        Self::handle_result(result).map(|_| Self { ptr })
    }

    /// Load an image from a window.
    ///
    /// # Parameters
    /// - `x`: The x position of the window.
    /// - `y`: The y position of the window.
    /// - `width`: The width of the window.
    /// - `height`: The height of the window.
    ///
    /// # Returns
    /// A new `Image` object. Or an error.
    pub fn from_window(x: i32, y: i32, width: i32, height: i32) -> Result<Self, ImageError> {
        let ptr = unsafe { ege_newimage() };
        let result = unsafe { ege_getimage(ptr, x, y, width, height) };
        Self::handle_result(result).map(|_| Self { ptr })
    }

    /// Load an image from another image.
    ///
    /// # Parameters
    /// - `image`: The source image.
    /// - `x`: The x position of the source image.
    /// - `y`: The y position of the source image.
    /// - `width`: The width of the source image.
    /// - `height`: The height of the source image.
    ///
    /// # Returns
    /// A new `Image` object. Or an error.
    pub fn from_image(
        image: &Image,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, ImageError> {
        let ptr = unsafe { ege_newimage() };
        let result = unsafe { ege_getimage1(ptr, image.ptr, x, y, width, height) };
        Self::handle_result(result).map(|_| Self { ptr })
    }

    /// Save an image to a file.
    ///
    /// # Parameters
    /// - `filename`: The filename of the image.
    /// - `with_alpha`: Whether to save the alpha channel.
    ///
    /// # Note
    /// It only supports BMP and PNG formats.
    pub fn save<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        with_alpha: bool,
    ) -> Result<(), ImageError> {
        let path = path
            .as_ref()
            .to_str()
            .map_or_else(|| Err(ImageError::PathParserError), |s| Ok(s))?
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let result = unsafe { ege_saveimage1(self.ptr, path.as_ptr(), with_alpha) };
        Self::handle_result(result)
    }
}

impl Image {
    /// Resize the image.
    ///
    /// # Parameters
    /// - `width`: The new width of the image.
    /// - `height`: The new height of the image.
    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), ImageError> {
        let result = unsafe { ege_resize(self.ptr, width, height) };
        Self::handle_result(result)
    }

    pub unsafe fn resize_f(&mut self, width: i32, height: i32) -> Result<(), ImageError> {
        let result = unsafe { ege_resize_f(self.ptr, width, height) };
        Self::handle_result(result)
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let mut img = Image::new(self.getwidth(), self.getheight());
        img.drawimage(self, 0, 0);
        img
    }
}

use crate::ARGB;

pub enum TemplateMode {
    White,
    Black,
    Ignore,
    DotCare,
}

#[bitmask_enum::bitmask]
pub enum ApplyMask {
    Red,
    Blue,
    Green,
    All = 15,
}

impl Image {
    pub fn transform(&mut self, trans: impl Fn(ARGB) -> ARGB) {
        for pixel in self.getbuffer_mut().iter_mut() {
            *pixel = trans(*pixel);
        }
    }

    pub fn template<const N: usize>(
        &mut self,
        mask: Mat<f32, N, N>,
        apply: ApplyMask,
        mode: TemplateMode,
    ) -> Self {
        let width = self.getwidth();
        let height = self.getheight();
        let mut img = Image::new(width, height);
        let src = self.getbuffer();
        let dst = img.getbuffer_mut();
        for i in 0..height {
            for j in 0..width {
                let (mut sum_red, mut sum_green, mut sum_blue) = (0.0, 0.0, 0.0);
                let mut is_dont_care = false;
                'dor_care: for m in 0..N as u32 {
                    for n in 0..N as u32 {
                        let x = (i + m) as i32 - N as i32 / 2;
                        let y = (j + n) as i32 - N as i32 / 2;
                        let c;
                        if x < 0 || x >= height as i32 || y < 0 || y >= width as i32 {
                            match mode {
                                TemplateMode::White => c = 0xFFFFFF,
                                TemplateMode::Black => c = 0x0,
                                TemplateMode::Ignore => continue,
                                TemplateMode::DotCare => {
                                    is_dont_care = true;
                                    break 'dor_care;
                                }
                            }
                        } else {
                            c = src[(x as u32 * width + y as u32) as usize];
                        }
                        if apply.contains(ApplyMask::Red) {
                            sum_red += mask[m as usize][n as usize] * ((c >> 16) & 0xFF) as f32;
                        }
                        if apply.contains(ApplyMask::Green) {
                            sum_green += mask[m as usize][n as usize] * ((c >> 8) & 0xFF) as f32;
                        }
                        if apply.contains(ApplyMask::Blue) {
                            sum_blue += mask[m as usize][n as usize] * (c & 0xFF) as f32;
                        }
                    }
                }
                if !is_dont_care {
                    let red = sum_red.max(0.0).min(255.0) as u8;
                    let green = sum_green.max(0.0).min(255.0) as u8;
                    let blue = sum_blue.max(0.0).min(255.0) as u8;
                    let alpha = 0xFF;
                    dst[(i * width + j) as usize] = ((alpha as u32) << 24)
                        | ((red as u32) << 16)
                        | ((green as u32) << 8)
                        | blue as u32;
                }
            }
        }
        img
    }
}
