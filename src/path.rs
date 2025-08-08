use std::ptr::null;

use xege_ffi::*;

use crate::{DrawableDevice, Point, Rect, mat::IntoEGEMatrix};

/// Graphics path.
#[derive(Debug)]
pub struct Path {
    pub(crate) ptr: *mut ege_ege_path,
}

impl Path {
    /// Create a new graphics path.
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

/// Path fill mode.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PathFillMode {
    /// Default fill mode.
    Default = ege_fill_mode_FILLMODE_DEFAULT,
    /// Winding fill mode.
    Winding = ege_fill_mode_FILLMODE_WINDING,
    /// Alternate fill mode.
    Alternate = ege_fill_mode_FILLMODE_ALTERNATE,
}

/// Font styles.
#[bitmask_enum::bitmask(i32)]
pub enum FontStyle {
    Black = ege_font_styles_FONTSTYLE_BOLD,
    Underline = ege_font_styles_FONTSTYLE_UNDERLINE,
    Italic = ege_font_styles_FONTSTYLE_ITALIC,
    StrikeOut = ege_font_styles_FONTSTYLE_STRIKEOUT,
}

impl Path {
    /// Start a new subpath.
    pub fn start(&mut self) {
        unsafe { ege_ege_path_start(self.ptr) };
    }

    /// Close the current subpath.
    pub fn close(&mut self) {
        unsafe { ege_ege_path_close(self.ptr) };
    }

    /// Close all subpaths.
    pub fn closeall(&mut self) {
        unsafe { ege_ege_path_closeall(self.ptr) };
    }

    /// Reset the path.
    pub fn reset(&mut self) {
        unsafe { ege_ege_path_reset(self.ptr) };
    }

    /// Reverses the order of the points in the path.
    pub fn reverse(&mut self) {
        unsafe { ege_ege_path_reverse(self.ptr) };
    }

    /// Set the fill mode.
    pub fn setfillmode(&mut self, mode: PathFillMode) {
        unsafe { ege_ege_path_setfillmode(self.ptr, mode as i32) };
    }

    /// Widen the path.
    /// 
    /// # Parameters
    /// - `width`: The width of the stroke.
    /// - `matrix`: The transformation matrix.
    /// - `flatness`: The flatness of the stroke.
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

    /// Flatten the path.
    /// 
    /// # Parameters
    /// - `matrix`: The transformation matrix.
    /// - `flatness`: The flatness of the stroke.
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

    /// Warp the path.
    /// 
    /// # Parameters
    /// - `points`: The control points.
    /// - `rect`: The destination rectangle.
    /// - `matrix`: The transformation matrix.
    /// - `flatness`: The flatness of the stroke.
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

    /// Convert the path to the outline of the path.
    /// 
    /// # Parameters
    /// - `matrix`: The transformation matrix.
    /// - `flatness`: The flatness of the stroke.
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

    /// Is the point in the path?
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the point.
    /// - `y`: The y-coordinate of the point.
    /// - `image`: The image to test against.
    pub fn inpath(&self, x: f32, y: f32, image: Option<&impl DrawableDevice>) -> bool {
        if let Some(device) = image {
            unsafe { ege_ege_path_inpath1(self.ptr, x, y, device.const_ptr()) }
        } else {
            unsafe { ege_ege_path_inpath(self.ptr, x, y) }
        }
    }

    /// Is the point on the stroke of the path?
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the point.
    /// - `y`: The y-coordinate of the point.
    /// - `image`: The image to test against.
    pub fn instroke(&self, x: f32, y: f32, image: Option<&impl DrawableDevice>) -> bool {
        if let Some(device) = image {
            unsafe { ege_ege_path_instroke1(self.ptr, x, y, device.const_ptr()) }
        } else {
            unsafe { ege_ege_path_instroke(self.ptr, x, y) }
        }
    }

    /// Gets the last point of the path.
    pub fn lastpoint(&self) -> Point<f32> {
        let p = unsafe { ege_ege_path_lastpoint(self.ptr) };
        Point { x: p.x, y: p.y }
    }

    /// Get the number of points in the path.
    pub fn pointcount(&self) -> i32 {
        unsafe { ege_ege_path_pointcount(self.ptr) }
    }

    /// Get the bounding box of the path.
    /// 
    /// # Parameters
    /// - `matrix`: The transformation matrix.
    /// - `image`: The image to test against.
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

    /// Transform the path.
    /// 
    /// # Parameters
    /// - `matrix`: The transformation matrix.
    pub fn transform(&mut self, matrix: impl IntoEGEMatrix) {
        let mat = matrix.into_ege_matrix();
        unsafe { ege_ege_path_transform(self.ptr, &mat) };
    }

    /// Add a path to the current path.
    /// 
    /// # Parameters
    /// - `other`: The other path.
    /// - `connnet`: Whether to connect the paths.
    pub fn addpath(&mut self, other: &Self, connnet: bool) {
        unsafe { ege_ege_path_addpath(self.ptr, other.ptr, connnet) };
    }

    /// Add a line to the current path.
    /// 
    /// # Parameters
    /// - `x1`: The x-coordinate of the first point.
    /// - `y1`: The y-coordinate of the first point.
    /// - `x2`: The x-coordinate of the second point.
    /// - `y2`: The y-coordinate of the second point.
    pub fn addline(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { ege_ege_path_addline(self.ptr, x1, y1, x2, y2) };
    }

    /// Add a arc to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the center.
    /// - `y`: The y-coordinate of the center.
    /// - `width`: The width of the arc.
    /// - `height`: The height of the arc.
    /// - `start`: The start angle in radians.
    /// - `sweep`: The sweep angle in radians.
    pub fn addarc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_path_addarc(self.ptr, x, y, width, height, start, sweep) };
    }

    /// Add a polyline to the current path.
    /// 
    /// # Parameters
    /// - `points`: The points of the polyline.
    pub fn addpolyline(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addpolyline(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    /// Add a bezier curve to the current path.
    /// 
    /// # Parameters
    /// - `points`: The points of the bezier curve.
    pub fn addbezier(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 4 && points.len() % 3 == 1);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addbezier(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    /// Add a curve to the current path.
    /// 
    /// # Parameters
    /// - `points`: The points of the curve.
    /// - `tension`: The tension of the curve.
    pub fn addcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addcurve1(self.ptr, points.len() as i32, points.as_ptr(), tension) };
    }

    /// Add a circle to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the center.
    /// - `y`: The y-coordinate of the center.
    /// - `radius`: The radius of the circle.
    pub fn addcircle(&mut self, x: f32, y: f32, radius: f32) {
        unsafe { ege_ege_path_addcircle(self.ptr, x, y, radius) };
    }

    /// Add a rectangle to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the top-left corner.
    /// - `y`: The y-coordinate of the top-left corner.
    /// - `width`: The width of the rectangle.
    /// - `height`: The height of the rectangle.
    pub fn addrect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe { ege_ege_path_addrect(self.ptr, x, y, width, height) };
    }

    /// Add an ellipse to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the center.
    /// - `y`: The y-coordinate of the center.
    /// - `width`: The width of the ellipse.
    /// - `height`: The height of the ellipse.
    pub fn addellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe { ege_ege_path_addellipse(self.ptr, x, y, width, height) };
    }

    /// Add a pie to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the center.
    /// - `y`: The y-coordinate of the center.
    /// - `width`: The width of the pie.
    /// - `height`: The height of the pie.
    /// - `start`: The start angle in radians.
    /// - `sweep`: The sweep angle in radians.
    pub fn addpie(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_path_addpie(self.ptr, x, y, width, height, start, sweep) };
    }

    /// Add a polygon to the current path.
    /// 
    /// # Parameters
    /// - `points`: The points of the polygon.
    pub fn addpolygon(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_path_addpolygon(self.ptr, points.len() as i32, points.as_ptr()) };
    }

    /// Add a closed curve to the current path.
    /// 
    /// # Parameters
    /// - `points`: The points of the curve.
    /// - `tension`: The tension of the curve.
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

    /// Add a text to the current path.
    /// 
    /// # Parameters
    /// - `x`: The x-coordinate of the left edge of the text.
    /// - `y`: The y-coordinate of the top edge of the text.
    /// - `text`: The text to add.
    /// - `font`: The font to use.
    /// - `size`: The size of the font.
    /// - `style`: The font styles.
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
