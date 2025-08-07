use std::fmt::Display;
use xege_ffi::*;

#[derive(Debug)]
pub struct Image {
    ptr: *mut ege_IMAGE,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image({:p})", self.ptr)
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
}