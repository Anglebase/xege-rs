use xege_ffi::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Window(pub(crate) *mut ());

impl Window {
    /// Move Window to (x, y) relative the screen.
    /// 
    /// # Parameters
    /// * `x` - The x coordinate of the new position.
    /// * `y` - The y coordinate of the new position.
    /// * `redraw` - If true, the window will be redrawn.
    pub fn move_to(&mut self, x: i32, y: i32, redraw: bool) {
        unsafe { ege_movewindow(x, y, redraw) };
    }

    /// Resize the window to (width, height).
    /// 
    /// # Parameters
    /// * `width` - The new width of the window.
    /// * `height` - The new height of the window.
    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe { ege_resizewindow(width, height) };
    }

    /// Flushes the window.
    pub fn flush(&mut self) {
        unsafe { ege_flushwindow() };
    }

    /// Show the window.
    /// 
    /// # Note
    /// If the initialization mode contains `Init::Hide`,
    /// then this function needs to be called for the window to display.
    pub fn show(&mut self) {
        unsafe { ege_showwindow() };
    }

    /// Hide the window.
    pub fn hide(&mut self) {
        unsafe { ege_hidewindow() };
    }
}