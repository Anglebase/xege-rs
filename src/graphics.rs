use crate::color::IntoARGB;
use crate::mat::IntoEGEMatrix;
use crate::{Color, Image, Path};
use crate::{ImageError, enums::*};
use xege_ffi::*;

/// Base trait for drawable devices.
pub trait DrawableDevice {
    fn mut_ptr(&mut self) -> *mut ege_IMAGE;
    fn const_ptr(&self) -> *const ege_IMAGE;

    #[allow(non_snake_case)]
    fn getHDC(&self) -> HDC {
        unsafe { ege_getHDC(self.const_ptr()) }
    }
}

/// Content related to the drawing environment.
pub trait GraphicsEnvironment: DrawableDevice {
    /// Clear the device.
    fn clear(&mut self) {
        unsafe { ege_cleardevice(self.mut_ptr()) };
    }

    /// Set current fill color.
    ///
    /// # Parameters
    /// * `color` - The color to set.
    fn setfillcolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_setfillcolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current color.
    ///
    /// # Parameters
    /// * `color` - The color to set.
    fn setcolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_setcolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current background color.
    /// 
    /// # Parameters
    /// * `color` - The color to set.
    fn setbkcolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_setbkcolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current background mode.
    /// 
    /// # Parameters
    /// * `mode` - The mode to set.
    fn setbkmode(&mut self, mode: BkMode) {
        unsafe { ege_setbkmode(mode as i32, self.mut_ptr()) };
    }

    /// Get current background color.
    /// 
    /// # Return
    /// The color of the background.
    fn getbkcolor(&self) -> Color {
        let argb = unsafe { ege_getbkcolor(self.const_ptr()) };
        Color::from_argb(argb)
    }

    /// Set current line color.
    ///
    /// # Parameters
    /// * `color` - The color to set.
    fn setlinecolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_setlinecolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current fill style.
    ///
    /// # Parameters
    /// * `pattern` - The pattern to set.
    /// * `color` - The color to set.
    fn setfillstyle(&mut self, pattern: FillPattern, color: impl IntoARGB) {
        unsafe { ege_setfillstyle(pattern as i32, color.into_argb(), self.mut_ptr()) };
    }

    /// Set current line style.
    ///
    /// # Parameters
    /// * `style` - The style to set.
    /// * `width` - The line width to set.
    fn setlinestyle(&mut self, style: LineStyle, width: i32) {
        if let LineStyle::UserDef(user_def) = style {
            unsafe { ege_setlinestyle(style.into(), user_def, width, self.mut_ptr()) };
        } else {
            unsafe { ege_setlinestyle(style.into(), 0, width, self.mut_ptr()) };
        }
    }

    /// Get current color.
    /// 
    /// # Return
    /// The color of the environment.
    fn getcolor(&self) -> Color {
        let argb = unsafe { ege_getcolor(self.const_ptr()) };
        Color::from_argb(argb)
    }

    /// Get current text color.
    ///
    /// # Return
    /// The color of the text.
    fn gettextcolor(&self) -> Color {
        let argb = unsafe { ege_gettextcolor(self.const_ptr()) };
        Color::from_argb(argb)
    }

    /// Set current text color.
    /// 
    /// # Parameters
    /// * `color` - The color to set.
    fn settextcolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_settextcolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current font background color.
    /// 
    /// # Parameters
    /// * `color` - The color to set.
    fn setfontbkcolor(&mut self, color: impl IntoARGB) {
        unsafe { ege_setfontbkcolor(color.into_argb(), self.mut_ptr()) };
    }

    /// Set current line cap.
    ///
    /// # Parameters
    /// * `start` - The start cap to set.
    /// * `end` - The end cap to set.
    fn setlinecap(&mut self, start: LineCap, end: LineCap) {
        unsafe { ege_setlinecap1(start as i32, end as i32, self.mut_ptr()) };
    }

    /// Set current line join.
    ///
    /// # Parameters
    /// * `join` - The join to set.
    /// * `limit` - The limit to set.
    fn setlinejoin(&mut self, join: LineJoin, limit: f32) {
        unsafe { ege_setlinejoin1(join as i32, limit, self.mut_ptr()) };
    }

    /// Set current line width.
    ///
    /// # Parameters
    /// * `width` - The width to set.
    fn setlinewidth(&mut self, width: f32) {
        unsafe { ege_setlinewidth(width, self.mut_ptr()) };
    }

    /// Get current fill color.
    ///
    /// # Return
    /// The color of the fill.
    fn getfillcolor(&self) -> Color {
        let argb = unsafe { ege_getfillcolor(self.const_ptr()) };
        Color::from_argb(argb)
    }

    /// Get current line style.
    ///
    /// # Return
    /// The style of the line and the width of the line: `(style, width)`.
    fn getlinestyle(&self) -> (LineStyle, i32) {
        let (mut linestyle, mut pattern, mut thickness) = (0i32, 0u16, 0i32);
        unsafe {
            ege_getlinestyle(
                &mut linestyle as _,
                &mut pattern as _,
                &mut thickness as _,
                self.const_ptr(),
            );
        }
        let style = if linestyle == ege_line_styles_USERBIT_LINE {
            LineStyle::UserDef(pattern)
        } else {
            LineStyle::try_from(linestyle).unwrap()
        };
        (style, thickness)
    }

    /// Get current line cap style.
    ///
    /// # Return
    /// The start and end cap of the line: `(start, end)`.
    fn getlinecap(&self) -> (LineCap, LineCap) {
        let (mut start, mut end) = (0i32, 0i32);
        unsafe { ege_getlinecap(&mut start as _, &mut end as _, self.const_ptr()) }
        (
            LineCap::try_from(start).unwrap(),
            LineCap::try_from(end).unwrap(),
        )
    }

    /// Get current line join style.
    ///
    /// # Return
    /// The join and limit of the line: `(join, limit)`.
    fn getlinejoin(&self) -> (LineJoin, f32) {
        let (mut join, mut limit) = (0i32, 0f32);
        unsafe { ege_getlinejoin(&mut join as _, &mut limit as _, self.const_ptr()) }
        (LineJoin::try_from(join).unwrap(), limit)
    }

    /// Get device width.
    fn getwidth(&self) -> i32 {
        unsafe { ege_getwidth(self.const_ptr()) }
    }

    /// Get device height.
    fn getheight(&self) -> i32 {
        unsafe { ege_getheight(self.const_ptr()) }
    }

    /// Get current x position.
    fn getx(&self) -> i32 {
        unsafe { ege_getx(self.const_ptr()) }
    }

    /// Get current y position.
    fn gety(&self) -> i32 {
        unsafe { ege_gety(self.const_ptr()) }
    }

    /// Move the current position by relative values.
    ///
    /// # Parameters
    /// * `dx` - The relative x position.
    /// * `dy` - The relative y position.
    fn moverel(&mut self, dx: i32, dy: i32) {
        unsafe { ege_moverel(dx, dy, self.mut_ptr()) };
    }

    /// Move the current position to absolute values.
    ///
    /// # Parameters
    /// * `x` - The absolute x position.
    /// * `y` - The absolute y position.
    fn moveto(&mut self, x: i32, y: i32) {
        unsafe { ege_moveto(x, y, self.mut_ptr()) };
    }

    /// Get current font.
    ///
    /// # Return
    /// The current font.
    fn getfont(&self) -> Font {
        let mut lfont = LOGFONTW {
            lfHeight: 0,
            lfWidth: 0,
            lfEscapement: 0,
            lfOrientation: 0,
            lfWeight: 0,
            lfItalic: 0,
            lfUnderline: 0,
            lfStrikeOut: 0,
            lfCharSet: 0,
            lfOutPrecision: 0,
            lfClipPrecision: 0,
            lfQuality: 0,
            lfPitchAndFamily: 0,
            lfFaceName: [0; 32],
        };
        unsafe { ege_getfont(&mut lfont as _, self.const_ptr()) };
        Font::try_from(lfont).unwrap()
    }

    /// Set current font.
    ///
    /// # Parameters
    /// * `font` - The font to set.
    fn setfont(&mut self, font: Font) {
        let lfont = font.into();
        unsafe { ege_setfont6(&lfont as _, self.mut_ptr()) };
    }

    /// Get text height.
    ///
    /// # Parameters
    /// * `text` - The text to measure.
    ///
    /// # Return
    /// The height of the text.
    fn textheight(&mut self, text: &str) -> i32 {
        let wchar = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_textheight1(wchar.as_ptr(), self.mut_ptr()) }
    }

    /// Get text width.
    ///
    /// # Parameters
    /// * `text` - The text to measure.
    ///
    /// # Return
    /// The width of the text.
    fn textwidth(&mut self, text: &str) -> i32 {
        let wchar = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_textwidth1(wchar.as_ptr(), self.mut_ptr()) }
    }

    /// Set Text alignment.
    ///
    /// # Parameters
    /// * `horiz` - The horizontal alignment.
    /// * `vert` - The vertical alignment.
    fn settextjustify(&mut self, horiz: TextHAlign, vert: TextVAlign) {
        unsafe { ege_settextjustify(horiz as i32, vert as i32, self.mut_ptr()) };
    }

    /// Get image pixel buffer pointer.
    ///
    /// # Return
    /// Pointer to the starting position of the image data memory.
    /// Each `u32` represents a pixel in `ARGB` format.
    fn getbuffer(&mut self) -> *mut u32 {
        unsafe { ege_getbuffer(self.mut_ptr()) }
    }

    /// Set global alpha transparency.
    fn set_alpha(&mut self, alpha: u8) {
        unsafe { ege_ege_setalpha(alpha as _, self.mut_ptr()) };
    }

    /// Set transformation matrix.
    ///
    /// # Parameters
    /// * `matrix` - The transformation matrix.
    fn set_transform(&mut self, matrix: impl IntoEGEMatrix) {
        let mat = matrix.into_ege_matrix();
        unsafe { ege_ege_set_transform(&mat, self.mut_ptr()) };
    }
}

/// Point and Color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelPoint<U: IntoARGB, T = i32> {
    pub x: T,
    pub y: T,
    pub color: U,
}

/// Line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

/// Point
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T = i32> {
    pub x: T,
    pub y: T,
}

/// Ordinary drawing functions.
///
/// # Note
/// They do not support anti aliasing and transparent channels.
pub trait Draw: DrawableDevice {
    /// Set ROP2.
    ///
    /// # Parameters
    /// * `gen_rop2`: The ROP2 generator.
    ///
    /// # Note
    /// The `gen_rop2` accepts two `u8`:
    ///     - `Pen`: current pen color.
    ///     - `Dst`: Screen color.
    ///
    /// * This function only valid for trait `Draw`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use xege::*;
    ///
    /// let mut xege = initgraph(640, 480, Init::Default).unwarp();
    /// // DPan
    /// xege.setwritemode(|pen, dst| pen & dst);
    /// ```
    fn setwritemode(&mut self, gen_rop2: impl Fn(u8, u8) -> u8) {
        let mask = gen_rop2(0b1100, 0b1010) + 1;
        unsafe { ege_setwritemode(mask as _, self.mut_ptr()) };
    }

    /// Get the color of the pixel at the specified position.
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    ///
    /// # Return
    /// The color of the pixel.
    fn getpixel(&self, x: i32, y: i32) -> Color {
        let argb = unsafe { ege_getpixel(x, y, self.const_ptr()) };
        Color::from_argb(argb)
    }

    /// Draw a pixel at the specified position.
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `color` - The color to draw.
    fn putpixel(&mut self, x: i32, y: i32, color: impl IntoARGB) {
        unsafe { ege_putpixel(x, y, color.into_argb(), self.mut_ptr()) };
    }

    /// Set the color of pixels while preserving the original alpha value
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `color` - The color to draw.
    fn putpixel_savealpha(&mut self, x: i32, y: i32, color: impl IntoARGB) {
        unsafe { ege_putpixel_savealpha(x, y, color.into_argb(), self.mut_ptr()) };
    }

    /// Draw pixel points
    /// Mix according to alpha, and retain the background color alpha in the mixed result.
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `color` - The color to draw.
    fn putpixel_withalpha(&mut self, x: i32, y: i32, color: impl IntoARGB) {
        unsafe { ege_putpixel_withalpha(x, y, color.into_argb(), self.mut_ptr()) };
    }

    /// Draw pixel points
    /// Mix based on alpha
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `color` - The color to draw.
    /// * `factor` - The alpha factor.
    fn putpixel_alphablend(&mut self, x: i32, y: i32, color: impl IntoARGB, factor: u8) {
        unsafe { ege_putpixel_alphablend1(x, y, color.into_argb(), factor, self.mut_ptr()) };
    }

    /// Draw multiple pixels
    ///
    /// # Parameters
    /// * `array` - The array of pixels to draw.
    fn putpixels<T: IntoARGB + Copy>(&mut self, array: &[PixelPoint<T>]) {
        let array = array
            .into_iter()
            .map(|&PixelPoint { x, y, color }| {
                let argb = color.into_argb();
                let argb = unsafe { *(&argb as *const u32 as *const i32) };
                [x, y, argb]
            })
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_putpixels(array.len() as _, array.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a arc.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle in degrees.
    /// * `end` - The end angle in degrees.
    /// * `radius` - The radius of the arc.
    fn arc(&mut self, x: i32, y: i32, start: i32, end: i32, radius: i32) {
        unsafe { ege_arc(x, y, start, end, radius, self.mut_ptr()) };
    }

    /// Draw a arc.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle in degrees.
    /// * `end` - The end angle in degrees.
    /// * `radius` - The radius of the arc.
    fn arcf(&mut self, x: f32, y: f32, start: f32, end: f32, radius: f32) {
        unsafe { ege_arcf(x, y, start, end, radius, self.mut_ptr()) };
    }

    /// Draw a ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle in degrees.
    /// * `end` - The end angle in degrees.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn ellipse(&mut self, x: i32, y: i32, start: i32, end: i32, rx: i32, ry: i32) {
        unsafe { ege_ellipse(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle in degrees.
    /// * `end` - The end angle in degrees.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn ellipsef(&mut self, x: f32, y: f32, start: f32, end: f32, rx: f32, ry: f32) {
        unsafe { ege_ellipsef(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a line between two points.
    ///
    /// # Parameters
    /// * `x1` - The x position of the first point.
    /// * `y1` - The y position of the first point.
    /// * `x2` - The x position of the second point.
    /// * `y2` - The y position of the second point.
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe { ege_line(x1, y1, x2, y2, self.mut_ptr()) };
    }

    /// Draw a line between current position and the specified relative coordinates.
    ///
    /// # Parameters
    /// * `dx` - The relative x position.
    /// * `dy` - The relative y position.
    fn linerel(&mut self, dx: i32, dy: i32) {
        unsafe { ege_linerel(dx, dy, self.mut_ptr()) };
    }

    /// Draw a line between current position and the specified absolute coordinates.
    ///
    /// # Parameters
    /// * `x` - The absolute x position.
    /// * `y` - The absolute y position.
    fn lineto(&mut self, x: i32, y: i32) {
        unsafe { ege_lineto(x, y, self.mut_ptr()) };
    }

    /// Draw multiple lines.
    ///
    /// # Parameters
    /// * `lines` - The array of lines to draw.
    fn drawlines(&mut self, lines: &[Line]) {
        let lines = lines
            .into_iter()
            .map(|&Line { x1, y1, x2, y2 }| [x1, y1, x2, y2])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_drawlines(lines.len() as _, lines.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a polyline.
    ///
    /// # Parameters
    /// * `points` - The array of points to draw.
    fn polyline(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_polyline(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a polyline.
    ///
    /// # Parameters
    /// * `points` - The array of points to draw.
    fn drawpoly(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_drawpoly(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a bezier curve.
    ///
    /// # Parameters
    /// * `points` - The array of control points to draw.
    fn drawbezier(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 4 && points.len() % 3 == 1);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_drawbezier(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    fn rectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { ege_rectangle(left, top, right, bottom, self.mut_ptr()) };
    }

    /// Draw a fill rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    fn fillrect(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { ege_fillrect(left, top, right, bottom, self.mut_ptr()) };
    }

    /// Draw a solid rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    fn solidrectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { ege_solidrect(left, top, right, bottom, self.mut_ptr()) };
    }

    /// Draw a rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    fn bar(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        unsafe { ege_bar(left, top, right, bottom, self.mut_ptr()) };
    }

    /// Draw a block.
    ///
    /// # Parameters
    /// * `left` - The left position of the block.
    /// * `top` - The top position of the block.
    /// * `right` - The right position of the block.
    /// * `bottom` - The bottom position of the block.
    /// * `depth` - The depth of the block.
    /// * `topflag` - Whether to fill the top of the block.
    fn bar3d(&mut self, left: i32, top: i32, right: i32, bottom: i32, depth: i32, topflag: bool) {
        unsafe {
            ege_bar3d(
                left,
                top,
                right,
                bottom,
                depth,
                topflag as _,
                self.mut_ptr(),
            )
        };
    }

    /// Draw a polygon.
    ///
    /// # Parameters
    /// * `points` - The vertices of the polygon.
    fn polygon(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_polygon(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a filled polygon.
    ///
    /// # Parameters
    /// * `points` - The vertices of the polygon.
    fn fillpoly(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_fillpoly(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a solid polygon.
    ///
    /// # Parameters
    /// * `points` - The vertices of the polygon.
    fn solidpoly(&mut self, points: &[Point]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| [x, y])
            .flatten()
            .collect::<Vec<_>>();
        unsafe { ege_solidpoly(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a gradient polygon.
    ///
    /// # Parameters
    /// * `points` - The vertices of the polygon.
    fn fillpoly_gradient<T: IntoARGB + Copy>(&mut self, points: &[PixelPoint<T, f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&PixelPoint { x, y, color }| ege_ege_colpoint {
                x,
                y,
                color: color.into_argb(),
            })
            .collect::<Vec<_>>();
        unsafe { ege_fillpoly_gradient(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn circle(&mut self, x: i32, y: i32, radius: i32) {
        unsafe { ege_circle(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn circlef(&mut self, x: f32, y: f32, radius: f32) {
        unsafe { ege_circlef(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a filled circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn fillcircle(&mut self, x: i32, y: i32, radius: i32) {
        unsafe { ege_fillcircle(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a filled circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn fillcirclef(&mut self, x: f32, y: f32, radius: f32) {
        unsafe { ege_fillcirclef(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn fillellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32) {
        unsafe { ege_fillellipse(x, y, rx, ry, self.mut_ptr()) };
    }

    /// Draw a ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn fillellipsef(&mut self, x: f32, y: f32, rx: f32, ry: f32) {
        unsafe { ege_fillellipsef(x, y, rx, ry, self.mut_ptr()) };
    }

    /// Draw a solid circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn solidcircle(&mut self, x: i32, y: i32, radius: i32) {
        unsafe { ege_solidcircle(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a solid circle.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `radius` - The radius of the circle.
    fn solidcirclef(&mut self, x: f32, y: f32, radius: f32) {
        unsafe { ege_solidcirclef(x, y, radius, self.mut_ptr()) };
    }

    /// Draw a solid ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn solidellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32) {
        unsafe { ege_solidellipse(x, y, rx, ry, self.mut_ptr()) };
    }

    /// Draw a solid ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `rx` - The x radius of the ellipse.
    /// * `ry` - The y radius of the ellipse.
    fn solidellipsef(&mut self, x: f32, y: f32, rx: f32, ry: f32) {
        unsafe { ege_solidellipsef(x, y, rx, ry, self.mut_ptr()) };
    }

    /// Draw a pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn pie(&mut self, x: i32, y: i32, start: i32, end: i32, rx: i32, ry: i32) {
        unsafe { ege_pie(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn pief(&mut self, x: f32, y: f32, start: f32, end: f32, rx: f32, ry: f32) {
        unsafe { ege_pief(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a filled pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn fillpie(&mut self, x: i32, y: i32, start: i32, end: i32, rx: i32, ry: i32) {
        unsafe { ege_fillpie(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a filled pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn fillpief(&mut self, x: f32, y: f32, start: f32, end: f32, rx: f32, ry: f32) {
        unsafe { ege_fillpief(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a solid pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn solidpie(&mut self, x: i32, y: i32, start: i32, end: i32, rx: i32, ry: i32) {
        unsafe { ege_solidpie(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a solid pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pie.
    /// * `end` - The end angle of the pie.
    /// * `rx` - The x radius of the pie.
    /// * `ry` - The y radius of the pie.
    fn solidpief(&mut self, x: f32, y: f32, start: f32, end: f32, rx: f32, ry: f32) {
        unsafe { ege_solidpief(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a sector.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the sector.
    /// * `end` - The end angle of the sector.
    /// * `rx` - The x radius of the sector.
    /// * `ry` - The y radius of the sector.
    fn sector(&mut self, x: i32, y: i32, start: i32, end: i32, rx: i32, ry: i32) {
        unsafe { ege_sector(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a sector.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the sector.
    /// * `end` - The end angle of the sector.
    /// * `rx` - The x radius of the sector.
    /// * `ry` - The y radius of the sector.
    fn sectorf(&mut self, x: f32, y: f32, start: f32, end: f32, rx: f32, ry: f32) {
        unsafe { ege_sectorf(x, y, start, end, rx, ry, self.mut_ptr()) };
    }

    /// Draw a pieslice.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pieslice.
    /// * `end` - The end angle of the pieslice.
    /// * `radius` - The radius of the pieslice.
    fn pieslice(&mut self, x: i32, y: i32, start: i32, end: i32, radius: i32) {
        unsafe { ege_pieslice(x, y, start, end, radius, self.mut_ptr()) };
    }

    /// Draw a pieslice.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `start` - The start angle of the pieslice.
    /// * `end` - The end angle of the pieslice.
    /// * `radius` - The radius of the pieslice.
    fn pieslicef(&mut self, x: f32, y: f32, start: f32, end: f32, radius: f32) {
        unsafe { ege_pieslicef(x, y, start, end, radius, self.mut_ptr()) };
    }

    /// Draw a rounded rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    /// * `rx` - The x radius of the rounded corners.
    /// * `ry` - The y radius of the rounded corners.
    fn roundrect(&mut self, left: i32, top: i32, right: i32, bottom: i32, rx: i32, ry: i32) {
        unsafe { ege_roundrect1(left, top, right, bottom, rx, ry, self.mut_ptr()) };
    }

    /// Draw a fill rounded rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    /// * `rx` - The x radius of the rounded corners.
    /// * `ry` - The y radius of the rounded corners.
    fn fillroundrect(&mut self, left: i32, top: i32, right: i32, bottom: i32, rx: i32, ry: i32) {
        unsafe { ege_fillroundrect1(left, top, right, bottom, rx, ry, self.mut_ptr()) };
    }

    /// Draw a solid rounded rectangle.
    ///
    /// # Parameters
    /// * `left` - The left position of the rectangle.
    /// * `top` - The top position of the rectangle.
    /// * `right` - The right position of the rectangle.
    /// * `bottom` - The bottom position of the rectangle.
    /// * `rx` - The x radius of the rounded corners.
    /// * `ry` - The y radius of the rounded corners.
    fn solidroundrect(&mut self, left: i32, top: i32, right: i32, bottom: i32, rx: i32, ry: i32) {
        unsafe { ege_solidroundrect1(left, top, right, bottom, rx, ry, self.mut_ptr()) };
    }

    /// Specify the boundary color filling area.
    ///
    /// # Parameters
    /// * `x` - Fill the x-position of any point within the region.
    /// * `y` - Fill the y-position of any point within the region.
    /// * `color` - Fill the color of the boundary of the area.
    fn floodfill(&mut self, x: i32, y: i32, color: impl IntoARGB) {
        let color = color.into_argb();
        let color = unsafe { *(&color as *const _ as *const i32) };
        unsafe { ege_floodfill(x, y, color, self.mut_ptr()) };
    }

    /// Specify the surface color filling area.
    ///
    /// # Parameters
    /// * `x` - Fill the x-position of any point within the region.
    /// * `y` - Fill the y-position of any point within the region.
    /// * `color` - Fill the color of the surface of the area.
    fn floodfillsurface(&mut self, x: i32, y: i32, color: impl IntoARGB) {
        unsafe { ege_floodfillsurface(x, y, color.into_argb(), self.mut_ptr()) };
    }

    /// Output text at current position.
    ///
    /// # Parameters
    /// * `text` - The text to output.
    fn outtext(&mut self, text: &str) {
        let text = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_outtext1(text.as_ptr(), self.mut_ptr()) };
    }

    /// Output text at specified rectangle area.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    /// * `text` - The text to output.
    fn outtextrect(&mut self, x: i32, y: i32, w: i32, h: i32, text: &str) {
        let text = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_outtextrect1(x, y, w, h, text.as_ptr(), self.mut_ptr()) };
    }

    /// Output text at specified position `(x, y)`.
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `text` - The text to output.
    fn outtextxy(&mut self, x: i32, y: i32, text: &str) {
        let text = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_outtextxy1(x, y, text.as_ptr(), self.mut_ptr()) };
    }
}

/// High-quality drawing functions.
///
/// # Note
/// They support anti aliasing and transparent channels.
pub trait HighDraw: DrawableDevice {
    /// Enable anti aliasing.
    ///
    /// # Parameters
    /// * `enable` - Whether to enable anti aliasing.
    fn enable_aa(&mut self, enable: bool) {
        unsafe { ege_ege_enable_aa(enable as _, self.mut_ptr()) };
    }

    /// Draw a line.
    ///
    /// # Parameters
    /// * `x1` - The x position of the start point.
    /// * `y1` - The y position of the start point.
    /// * `x2` - The x position of the end point.
    /// * `y2` - The y position of the end point.
    fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { ege_ege_line(x1, y1, x2, y2, self.mut_ptr()) };
    }

    /// Draw a rectangle.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ege_ege_rectangle(x, y, w, h, self.mut_ptr()) };
    }

    /// Draw a rounded rectangle.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    /// * `radius` - The radius of the rounded corners.
    fn roundrect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32) {
        unsafe { ege_ege_roundrect(x, y, w, h, radius, self.mut_ptr()) };
    }

    /// Draw a rounded rectangle with different radii for each corner.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    /// * `r1` - The radius of the top-left corner.
    /// * `r2` - The radius of the top-right corner.
    /// * `r3` - The radius of the bottom-right corner.
    /// * `r4` - The radius of the bottom-left corner.
    fn roundrect_every(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        r1: f32,
        r2: f32,
        r3: f32,
        r4: f32,
    ) {
        unsafe { ege_ege_roundrect1(x, y, w, h, r1, r2, r3, r4, self.mut_ptr()) };
    }

    /// Draw an arc.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `w` - The width of the arc.
    /// * `h` - The height of the arc.
    /// * `start` - The start angle of the arc.
    /// * `sweep` - The sweep angle of the arc.
    fn arc(&mut self, x: f32, y: f32, w: f32, h: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_arc(x, y, w, h, start, sweep, self.mut_ptr()) };
    }

    /// Draw an ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `w` - The width of the ellipse.
    /// * `h` - The height of the ellipse.
    fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ege_ege_ellipse(x, y, w, h, self.mut_ptr()) };
    }

    /// Draw a pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `w` - The width of the pie.
    /// * `h` - The height of the pie.
    /// * `start` - The start angle of the pie.
    /// * `sweep` - The sweep angle of the pie.
    fn pie(&mut self, x: f32, y: f32, w: f32, h: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_pie(x, y, w, h, start, sweep, self.mut_ptr()) };
    }

    /// Draw a polyline.
    ///
    /// # Parameters
    /// * `points` - The points of the polyline.
    fn drawpoly(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_drawpoly(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a bezier curve.
    ///
    /// # Parameters
    /// * `points` - The control points of the bezier curve.
    fn bezier(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 4 && points.len() % 3 == 1);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_bezier(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a curve.
    ///
    /// # Parameters
    /// * `points` - The control points of the curve.
    /// * `tension` - The tension of the curve.
    fn drawcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 2);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_drawcurve1(points.len() as _, points.as_ptr(), tension, self.mut_ptr()) };
    }

    /// Draw a closed curve.
    ///
    /// # Parameters
    /// * `points` - The control points of the curve.
    /// * `tension` - The tension of the curve.
    fn drawclosedcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe {
            ege_ege_drawclosedcurve1(points.len() as _, points.as_ptr(), tension, self.mut_ptr())
        };
    }

    /// Draw a fill rectangle.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    fn fillrect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ege_ege_fillrect(x, y, w, h, self.mut_ptr()) };
    }

    /// Draw a fill rounded rectangle.
    ///
    /// # Parameters
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `w` - The width of the rectangle.
    /// * `h` - The height of the rectangle.
    /// * `radius` - The radius of the rounded corners.
    fn fillroundrect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32) {
        unsafe { ege_ege_fillroundrect(x, y, w, h, radius, self.mut_ptr()) };
    }

    /// Draw a filled polygon.
    ///
    /// # Parameters
    /// * `points` - The vertices of the polygon.
    fn fillpoly(&mut self, points: &[Point<f32>]) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe { ege_ege_fillpoly(points.len() as _, points.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a filled pie.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `w` - The width of the pie.
    /// * `h` - The height of the pie.
    /// * `start` - The start angle of the pie.
    /// * `sweep` - The sweep angle of the pie.
    fn fillpie(&mut self, x: f32, y: f32, w: f32, h: f32, start: f32, sweep: f32) {
        unsafe { ege_ege_fillpie(x, y, w, h, start, sweep, self.mut_ptr()) };
    }

    /// Draw a filled ellipse.
    ///
    /// # Parameters
    /// * `x` - The x position of the center.
    /// * `y` - The y position of the center.
    /// * `w` - The width of the ellipse.
    /// * `h` - The height of the ellipse.
    fn fillellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        unsafe { ege_ege_fillellipse(x, y, w, h, self.mut_ptr()) };
    }

    /// Draw a filled bezier curve.
    ///
    /// # Parameters
    /// * `points` - The control points of the bezier curve.
    /// * `tension` - The tension of the curve.
    fn fillclosedcurve(&mut self, points: &[Point<f32>], tension: f32) {
        #[cfg(debug_assertions)]
        assert!(points.len() >= 3);
        let points = points
            .into_iter()
            .map(|&Point { x, y }| ege_ege_point { x, y })
            .collect::<Vec<_>>();
        unsafe {
            ege_ege_fillclosedcurve1(points.len() as _, points.as_ptr(), tension, self.mut_ptr())
        };
    }

    /// Output text at position `(x, y)`.
    ///
    /// # Parameters
    /// * `x` - The x position.
    /// * `y` - The y position.
    /// * `text` - The text to output.
    fn outtextxy(&mut self, x: f32, y: f32, text: &str) {
        let text = text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        unsafe { ege_ege_outtextxy1(x, y, text.as_ptr(), self.mut_ptr()) };
    }

    /// Draw a path
    /// 
    /// # Parameters
    /// * `path` - The path to draw.
    fn drawpath(&mut self, path: &Path) {
        unsafe { ege_ege_drawpath(path.ptr, self.mut_ptr()) };
    }

    /// Draw a path at position `(x, y)`.
    /// 
    /// # Parameters
    /// * `path` - The path to draw.
    /// * `x` - The x position.
    /// * `y` - The y position.
    fn drawpath_at(&mut self, path: &Path, x: f32, y: f32) {
        unsafe { ege_ege_drawpath1(path.ptr, x, y, self.mut_ptr()) };
    }

    /// Draw a filled path.
    /// 
    /// # Parameters
    /// * `path` - The path to draw.
    fn fillpath(&mut self, path: &Path) {
        unsafe { ege_ege_fillpath(path.ptr, self.mut_ptr()) };
    }

    /// Draw a filled path at position `(x, y)`.
    /// 
    /// # Parameters
    /// * `path` - The path to draw.
    /// * `x` - The x position.
    /// * `y` - The y position.
    fn fillpath_at(&mut self, path: &Path, x: f32, y: f32) {
        unsafe { ege_ege_fillpath1(path.ptr, x, y, self.mut_ptr()) };
    }
}

fn rop3(gen_rop3: impl Fn(u32, u32, u32) -> u32) -> u32 {
    gen_rop3(0xF00000, 0xCC0000, 0xAA0000)
}

/// Rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T = i32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

pub trait ImageDraw: DrawableDevice {
    /// Draw an image.
    ///
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x` - The x position of the top-left corner.
    /// * `y` - The y position of the top-left corner.
    /// * `gen_rop3` - A function to generate the ROP3 code.
    ///
    /// # Note
    /// The `gen_rop3` function takes three parameters:
    ///     - `Pen` mask.
    ///     - `Src` mask.
    ///     - `Dst` mask.
    ///
    /// For example, the `DPo` is `|P, S, D| D | P`, the `DPSao` is `|P, S, D| (S & P) | D`, and so on.
    fn putimage(&mut self, x: i32, y: i32, image: &Image, gen_rop3: impl Fn(u32, u32, u32) -> u32) {
        if self.mut_ptr().is_null() {
            unsafe { ege_putimage(x, y, image.const_ptr(), rop3(gen_rop3)) };
        } else {
            unsafe { ege_putimage3(self.mut_ptr(), x, y, image.const_ptr(), rop3(gen_rop3)) };
        }
    }

    /// Draw an image with a specified size.
    ///
    /// # Parameters
    /// * `dest` - The destination rectangle.
    /// * `image` - The image to draw.
    /// * `x_src` - The x position of the top-left corner of the source image.
    /// * `y_src` - The y position of the top-left corner of the source image.
    /// * `gen_rop3` - A function to generate the ROP3 code.
    fn putimage_with_size(
        &mut self,
        dest: Rect,
        image: &Image,
        x_src: i32,
        y_src: i32,
        gen_rop3: impl Fn(u32, u32, u32) -> u32,
    ) {
        if self.mut_ptr().is_null() {
            unsafe {
                ege_putimage1(
                    dest.x,
                    dest.y,
                    dest.width,
                    dest.height,
                    image.const_ptr(),
                    x_src,
                    y_src,
                    rop3(gen_rop3),
                );
            }
        } else {
            unsafe {
                ege_putimage4(
                    self.mut_ptr(),
                    dest.x,
                    dest.y,
                    dest.width,
                    dest.height,
                    image.const_ptr(),
                    x_src,
                    y_src,
                    rop3(gen_rop3),
                );
            }
        }
    }

    /// Draw an image with a specified scale.
    ///
    /// # Parameters
    /// * `dest` - The destination rectangle.
    /// * `image` - The image to draw.
    /// * `src` - The source rectangle.
    /// * `gen_rop3` - A function to generate the ROP3 code.
    fn putimage_with_scale(
        &mut self,
        dest: Rect,
        image: &Image,
        src: Rect,
        gen_rop3: impl Fn(u32, u32, u32) -> u32,
    ) {
        if self.mut_ptr().is_null() {
            unsafe {
                ege_putimage2(
                    dest.x,
                    dest.y,
                    dest.width,
                    dest.height,
                    image.const_ptr(),
                    src.x,
                    src.y,
                    src.width,
                    src.height,
                    rop3(gen_rop3),
                );
            }
        } else {
            unsafe {
                ege_putimage5(
                    self.mut_ptr(),
                    dest.x,
                    dest.y,
                    dest.width,
                    dest.height,
                    image.const_ptr(),
                    src.x,
                    src.y,
                    src.width,
                    src.height,
                    rop3(gen_rop3),
                );
            }
        }
    }

    /// Draw an image with alpha.
    ///
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `dest` - The destination rectangle.
    /// * `src` - The source rectangle.
    /// * `smooth` - Whether to use smooth scaling.
    fn putimage_with_alpha(
        &mut self,
        image: &Image,
        dest: Rect,
        src: Rect,
        smooth: bool,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_withalpha1(
                self.mut_ptr(),
                image.const_ptr(),
                dest.x,
                dest.y,
                dest.width,
                dest.height,
                src.x,
                src.y,
                src.width,
                src.height,
                smooth,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with alpha blending and a specified alpha type.
    ///
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `dest` - The destination rectangle.
    /// * `alpha` - The alpha value.
    /// * `src` - The source rectangle.
    /// * `smooth` - Whether to use smooth scaling.
    /// * `alpha_type` - The alpha type.
    fn putimage_alphablender(
        &mut self,
        image: &Image,
        dest: Rect,
        alpha: u8,
        src: Rect,
        smooth: bool,
        alpha_type: AlphaType,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_alphablend3(
                self.mut_ptr(),
                image.const_ptr(),
                dest.x,
                dest.y,
                dest.width,
                dest.height,
                alpha,
                src.x,
                src.y,
                src.width,
                src.height,
                smooth,
                alpha_type as i32,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with alpha blending and a specified alpha type.
    ///
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_dest` - The x position of the top-left corner of the destination image.
    /// * `y_dest` - The y position of the top-left corner of the destination image.
    /// * `alpha` - The alpha image.
    /// * `src` - The source rectangle.
    fn putimage_alphafilter(
        &mut self,
        image: &Image,
        x_dest: i32,
        y_dest: i32,
        alpha: &Image,
        src: Rect,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_alphafilter(
                self.mut_ptr(),
                image.const_ptr(),
                x_dest,
                y_dest,
                alpha.const_ptr(),
                src.x,
                src.y,
                src.width,
                src.height,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with alpha blending and a specified alpha type.
    ///
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_dest` - The x position of the top-left corner of the destination image.
    /// * `y_dest` - The y position of the top-left corner of the destination image.
    /// * `transparent` - The alpha image.
    /// * `src` - The source rectangle.
    fn putimage_transparent(
        &mut self,
        image: &Image,
        x_dest: i32,
        y_dest: i32,
        transparent: impl IntoARGB,
        src: Rect,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_transparent(
                self.mut_ptr(),
                image.const_ptr(),
                x_dest,
                y_dest,
                transparent.into_argb(),
                src.x,
                src.y,
                src.width,
                src.height,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with alpha blending and a specified alpha type.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_dest` - The x position of the top-left corner of the destination image.
    /// * `y_dest` - The y position of the top-left corner of the destination image.
    /// * `transparent` - The alpha value.
    /// * `alpha` - The alpha value.
    /// * `src` - The source rectangle.
    fn putimage_alphatransparent(
        &mut self,
        image: &Image,
        x_dest: i32,
        y_dest: i32,
        transparent: impl IntoARGB,
        alpha: u8,
        src: Rect,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_alphatransparent(
                self.mut_ptr(),
                image.const_ptr(),
                x_dest,
                y_dest,
                transparent.into_argb(),
                alpha,
                src.x,
                src.y,
                src.width,
                src.height,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with rotation.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_dest` - The x position of the top-left corner of the destination image.
    /// * `y_dest` - The y position of the top-left corner of the destination image.
    /// * `x_center` - The x position of the center of the rotation.
    /// * `y_center` - The y position of the center of the rotation.
    /// * `radian` - The rotation angle in radian.
    /// * `use_alpha` - Whether to use alpha blending.
    /// * `alpha` - The alpha value.
    /// * `smooth` - Whether to use smooth scaling.
    fn putimage_rotate(
        &mut self,
        image: &Image,
        x_dest: i32,
        y_dest: i32,
        x_center: f32,
        y_center: f32,
        radian: f32,
        use_alpha: bool,
        alpha: Option<u8>,
        smooth: bool,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_rotate(
                self.mut_ptr(),
                image.const_ptr(),
                x_dest,
                y_dest,
                x_center,
                y_center,
                radian,
                use_alpha,
                if let Some(alpha) = alpha {
                    alpha as i32
                } else {
                    -1
                },
                smooth,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with rotation and scale.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_dest` - The x position of the top-left corner of the destination image.
    /// * `y_dest` - The y position of the top-left corner of the destination image.
    /// * `x_center` - The x position of the center of the rotation.
    /// * `y_center` - The y position of the center of the rotation.
    /// * `radian` - The rotation angle in radian.
    /// * `zoom` - The zoom factor.
    /// * `use_alpha` - Whether to use alpha blending.
    /// * `alpha` - The alpha value.
    /// * `smooth` - Whether to use smooth scaling.
    fn putimage_rotatezoom(
        &mut self,
        image: &Image,
        x_dest: i32,
        y_dest: i32,
        x_center: f32,
        y_center: f32,
        radian: f32,
        zoom: f32,
        use_alpha: bool,
        alpha: Option<u8>,
        smooth: bool,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_rotatezoom(
                self.mut_ptr(),
                image.const_ptr(),
                x_dest,
                y_dest,
                x_center,
                y_center,
                radian,
                zoom,
                use_alpha,
                if let Some(alpha) = alpha {
                    alpha as i32
                } else {
                    -1
                },
                smooth,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with rotation and scale with alpha blending.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x_center_dest` - The x position of the center of the destination image.
    /// * `y_center_dest` - The y position of the center of the destination image.
    /// * `src` - The source rectangle.
    /// * `x_center_src` - The x position of the center of the source image.
    /// * `y_center_src` - The y position of the center of the source image.
    /// * `transparent` - The alpha image.
    /// * `radian` - The rotation angle in radian.
    /// * `zoom` - The zoom factor.
    fn putimage_rotatetransparent(
        &mut self,
        image: &Image,
        x_center_dest: i32,
        y_center_dest: i32,
        src: Rect,
        x_center_src: i32,
        y_center_src: i32,
        transparent: impl IntoARGB,
        radian: f32,
        zoom: f32,
    ) -> Result<(), ImageError> {
        let result = unsafe {
            ege_putimage_rotatetransparent1(
                self.mut_ptr(),
                image.const_ptr(),
                x_center_dest,
                y_center_dest,
                src.x,
                src.y,
                src.width,
                src.height,
                x_center_src,
                y_center_src,
                transparent.into_argb(),
                radian,
                zoom,
            )
        };
        Image::handle_result(result)
    }

    /// Blur the image.
    /// 
    /// # Parameters
    /// * `intensity` - The intensity of the blur.
    /// * `alpha` - The light value.
    /// * `dest` - The destination rectangle.
    fn imagefilter_blurring(
        &mut self,
        intensity: i32,
        alpha: u32,
        dest: Option<Rect>,
    ) -> Result<(), ImageError> {
        let rect = dest.unwrap_or_else(|| Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        });
        let result = unsafe {
            ege_imagefilter_blurring(
                self.mut_ptr(),
                intensity,
                alpha as _,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        };
        Image::handle_result(result)
    }

    /// Draw an image with a specified position.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `x` - The x position.
    /// * `y` - The y position.
    fn drawimage(&mut self, image: &Image, x: i32, y: i32) {
        unsafe { ege_ege_drawimage(image.const_ptr(), x, y, self.mut_ptr()) };
    }

    /// Draw an image with scale.
    /// 
    /// # Parameters
    /// * `image` - The image to draw.
    /// * `dest` - The destination rectangle.
    /// * `src` - The source rectangle.
    fn drawimage_with_scale(&mut self, image: &Image, dest: Rect, src: Rect) {
        unsafe {
            ege_ege_drawimage1(
                image.const_ptr(),
                dest.x,
                dest.y,
                dest.width,
                dest.height,
                src.x,
                src.y,
                src.width,
                src.height,
                self.mut_ptr(),
            )
        };
    }
}

impl<T: DrawableDevice> GraphicsEnvironment for T {}
impl<T: DrawableDevice> Draw for T {}
impl<T: DrawableDevice> HighDraw for T {}
impl<T: DrawableDevice> ImageDraw for T {}
