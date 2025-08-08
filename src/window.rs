use xege_ffi::*;

use crate::{Key, KeyFlags, KeyMsg, MouseMsg, Point};

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

    pub fn keystate(&self, key: Key) -> bool {
        unsafe { ege_keystate(<Key as Into<u32>>::into(key) as i32) }
    }

    pub fn getchar(&self) -> Option<char> {
        if unsafe { ege_kbhit() != 0 } {
            char::from_u32(unsafe { ege_getch() as u32 })
        } else {
            None
        }
    }

    pub fn getmsg(&self) -> Option<(KeyMsg, KeyFlags)> {
        if unsafe { ege_kbmsg() != 0 } {
            let msg = unsafe { ege_getkey() };
            let keymsg = match msg.msg {
                xege_ffi::ege_key_msg_e_key_msg_down => KeyMsg::Down(Key::from(msg.key as u32)),
                xege_ffi::ege_key_msg_e_key_msg_up => KeyMsg::Up(Key::from(msg.key as u32)),
                xege_ffi::ege_key_msg_e_key_msg_char => {
                    KeyMsg::Char(char::from_u32(msg.key as u32)?)
                }
                _ => unreachable!(),
            };
            let mut flags = KeyFlags::none();
            if (msg.flags & xege_ffi::ege_key_flag_e_key_flag_shift as u32) != 0 {
                flags |= KeyFlags::Shift;
            }
            if (msg.flags & xege_ffi::ege_key_flag_e_key_flag_ctrl as u32) != 0 {
                flags |= KeyFlags::Ctrl;
            }
            if (msg.flags & xege_ffi::ege_key_flag_e_key_flag_first_down as u32) != 0 {
                flags |= KeyFlags::First;
            }
            Some((keymsg, flags))
        } else {
            None
        }
    }

    pub fn flushkey(&self) {
        unsafe { ege_flushkey() };
    }

    pub fn getmouse(&self) -> Option<MouseMsg> {
        if unsafe { ege_mousemsg() != 0 } {
            let msg = unsafe { ege_getmouse() };
            Some(MouseMsg { msg })
        } else {
            None
        }
    }

    pub fn mousepos(&self) -> Point {
        let mut p = Point { x: 0, y: 0 };
        unsafe { ege_mousepos(&mut p.x, &mut p.y) };
        p
    }
    
    pub fn showmouse(&self, show: bool) {
        unsafe { ege_showmouse(show as i32) };
    }

    pub fn flushmouse(&self) {
        unsafe { ege_flushmouse() };
    }
}
