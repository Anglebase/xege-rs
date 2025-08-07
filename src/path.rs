use std::ptr::null;

use xege_ffi::*;

use crate::{DrawableDevice, Point, Rect, mat::IntoEGEMatrix};

#[derive(Debug)]
pub struct Path {
    pub(crate) ptr: *mut ege_ege_path,
}

impl Path {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { ege_ege_path_create() },
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { ege_ege_path_destroy(self.ptr) };
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { ege_ege_path_clone(self.ptr) },
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PathFillMode {
    Default = ege_fill_mode_FILLMODE_DEFAULT,
    Winding = ege_fill_mode_FILLMODE_WINDING,
    Alternate = ege_fill_mode_FILLMODE_ALTERNATE,
}

#[bitmask_enum::bitmask]
pub enum FontStyle {
    Black,
    Underline,
    Italic,
    StrikeOut,
}

impl Path {
    pub fn start(&mut self) {
        unsafe { ege_ege_path_start(self.ptr) };
    }

    pub fn close(&mut self) {
        unsafe { ege_ege_path_close(self.ptr) };
    }

    pub fn closeall(&mut self) {
        unsafe { ege_ege_path_closeall(self.ptr) };
    }

    pub fn reset(&mut self) {
        unsafe { ege_ege_path_reset(self.ptr) };
    }

    pub fn reverse(&mut self) {
        unsafe { ege_ege_path_reverse(self.ptr) };
    }

    pub fn setfillmode(&mut self, mode: PathFillMode) {
        unsafe { ege_ege_path_setfillmode(self.ptr, mode as i32) };
    }

    pub fn widen(&mut self, width: f32, matrix: Option<impl IntoEGEMatrix>, flatness: f32) {
        let mut ptr = null();
        let mat;
        if let Some(matrix) = matrix {
            mat = matrix.into_ege_matrix();
            ptr = &mat as _;
        }
        unsafe {
            ege_ege_path_widen1(self.ptr, width, ptr, flatness);
        }
    }

    pub fn flatten(&mut self, matrix: Option<impl IntoEGEMatrix>, flatness: f32) {
        let mut ptr = null();
        let mat;
        if let Some(matrix) = matrix {
            mat = matrix.into_ege_matrix();
            ptr = &mat as _;
        }
        unsafe {
            ege_ege_path_flatten1(self.ptr, ptr, flatness);
        }
    }

    pub fn warp(
        &mut self,
        points: &[Point<f32>],
        rect: Rect<f32>,
        matrix: Option<impl IntoEGEMatrix>,
        flatness: f32,
    ) {
        let mut ptr = null();
        let mat;
        if let Some(matrix) = matrix {
            mat = matrix.into_ege_matrix();
            ptr = &mat as _;
        }
        assert!(points.len() == 3 || points.len() == 4);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();

        let rect = ege_ege_rect {
            x: rect.x,
            y: rect.y,
            w: rect.width,
            h: rect.height,
        };
        unsafe {
            ege_ege_path_warp1(
                self.ptr,
                points.as_ptr(),
                points.len() as i32,
                &rect,
                ptr,
                flatness,
            );
        }
    }

    pub fn outline(&mut self, matrix: Option<impl IntoEGEMatrix>, flatness: f32) {
        let mut ptr = null();
        let mat;
        if let Some(matrix) = matrix {
            mat = matrix.into_ege_matrix();
            ptr = &mat as _;
        }
        unsafe {
            ege_ege_path_outline1(self.ptr, ptr, flatness);
        }
    }

    pub fn inpath(&self, x: f32, y: f32, image: Option<&impl DrawableDevice>) -> bool {
        if let Some(device) = image {
            unsafe { ege_ege_path_inpath1(self.ptr, x, y, device.const_ptr()) }
        } else {
            unsafe { ege_ege_path_inpath(self.ptr, x, y) }
        }
    }

    pub fn instroke(&self, x: f32, y: f32, image: Option<&impl DrawableDevice>) -> bool {
        if let Some(device) = image {
            unsafe { ege_ege_path_instroke1(self.ptr, x, y, device.const_ptr()) }
        } else {
            unsafe { ege_ege_path_instroke(self.ptr, x, y) }
        }
    }

    pub fn lastpoint(&self) -> Point<f32> {
        let p = unsafe { ege_ege_path_lastpoint(self.ptr) };
        Point { x: p.x, y: p.y }
    }

    pub fn pointcount(&self) -> i32 {
        unsafe { ege_ege_path_pointcount(self.ptr) }
    }

    pub fn getbounds(
        &self,
        matrix: Option<impl IntoEGEMatrix>,
        image: Option<&impl DrawableDevice>,
    ) -> Rect<f32> {
        let mut ptr = null();
        let mat;
        if let Some(matrix) = matrix {
            mat = matrix.into_ege_matrix();
            ptr = &mat as _;
        }
        if let Some(device) = image {
            let r = unsafe { ege_ege_path_getbounds1(self.ptr, ptr, device.const_ptr()) };
            Rect {
                x: r.x,
                y: r.y,
                width: r.w,
                height: r.h,
            }
        } else {
            let r = unsafe { ege_ege_path_getbounds(self.ptr, ptr) };
            Rect {
                x: r.x,
                y: r.y,
                width: r.w,
                height: r.h,
            }
        }
    }

    pub fn transform(&mut self, matrix: impl IntoEGEMatrix) {
        let mat = matrix.into_ege_matrix();
        unsafe { ege_ege_path_transform(self.ptr, &mat) };
    }

    pub fn addpath(&mut self, other: &Self, connnet: bool) {
        unsafe { ege_ege_path_addpath(self.ptr, other.ptr, connnet) };
    }

    pub fn addline(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { ege_ege_path_addline(self.ptr, x1, y1, x2, y2) };
    }

    pub fn addarc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_path_addarc(self.ptr, x, y, width, height, start, sweep) };
    }

    pub fn addpolyline(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addpolyline(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    pub fn addbezier(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 4 && points.len() % 3 == 1);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addbezier(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    pub fn addcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addcurve1(self.ptr, points.len() as i32, points.as_ptr(), tension) };
    }

    pub fn addcircle(&mut self, x: f32, y: f32, radius: f32) {
        unsafe { ege_ege_path_addcircle(self.ptr, x, y, radius) };
    }

    pub fn addrect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe { ege_ege_path_addrect(self.ptr, x, y, width, height) };
    }

    pub fn addellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe { ege_ege_path_addellipse(self.ptr, x, y, width, height) };
    }

    pub fn addpie(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_path_addpie(self.ptr, x, y, width, height, start, sweep) };
    }

    pub fn addpolygon(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addpolygon(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    pub fn addclosedcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe {
            ege_ege_path_addclosedcurve1(self.ptr, points.len() as i32, points.as_ptr(), tension)
        };
    }

    pub fn addtext(&mut self, x: f32, y: f32, text: &str, font: &str, size: f32, style: FontStyle) {
        let text = text.encode_utf16().collect::<Vec<u16>>();
        let font = font.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe {
            ege_ege_path_addtext1(
                self.ptr,
                x,
                y,
                text.as_ptr(),
                size,
                text.len() as i32,
                font.as_ptr(),
                style.bits() as _,
            )
        };
    }
}
