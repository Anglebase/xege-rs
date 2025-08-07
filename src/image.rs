use std::fmt::Display;
use xege_ffi::*;

use crate::DrawableDevice;

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
}

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

    pub fn from_file(filename: &str) -> Result<Self, ImageError> {
        let ptr = unsafe { ege_newimage() };
        let filename = filename.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        let result = unsafe { ege_getimage3(ptr, filename.as_ptr(), 0, 0) };
        Self::handle_result(result).map(|_| Self { ptr })
    }

    pub fn from_window(x: i32, y: i32, width: i32, height: i32) -> Result<Self, ImageError> {
        let ptr = unsafe { ege_newimage() };
        let result = unsafe { ege_getimage(ptr, x, y, width, height) };
        Self::handle_result(result).map(|_| Self { ptr })
    }

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

    pub fn save(&self, filename: &str, with_alpha: bool) -> Result<(), ImageError> {
        let filename = filename.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        let result = unsafe { ege_saveimage1(self.ptr, filename.as_ptr(), with_alpha) };
        Self::handle_result(result)
    }
}

impl Image {
    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), ImageError> {
        let result = unsafe { ege_resize(self.ptr, width, height) };
        Self::handle_result(result)
    }
}
