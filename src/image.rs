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
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            ptr: unsafe { ege_newimage1(width, height) },
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
