use xege_ffi::*;

use crate::{Key, Point};

/// Key flags
#[bitmask_enum::bitmask]
pub enum KeyFlags {
    Shift,
    Ctrl,
    First,
}

/// Key event message
#[derive(Debug)]
pub enum KeyMsg {
    Down(Key),
    Up(Key),
    Char(char),
}

/// Mouse event message
#[derive(Debug)]
pub struct MouseMsg {
    pub(crate) msg: ege_mouse_msg,
}

impl MouseMsg {
    /// Get mouse position
    pub fn pos(&self) -> Point {
        Point {
            x: self.msg.x,
            y: self.msg.y,
        }
    }

    /// Is move event
    pub fn is_move(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_move
    }

    /// Is button down event
    pub fn is_down(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_down
    }

    /// Is button up event
    pub fn is_up(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_up
    }

    /// Is left button event
    pub fn is_left(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_left as u32 != 0
    }

    /// Is right button event
    pub fn is_right(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_right as u32 != 0
    }

    /// Is middle button event
    pub fn is_middle(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_mid as u32 != 0
    }

    /// Is wheel event
    pub fn is_wheel(&self) -> bool {
        self.msg.flags & ege_mouse_msg_e_mouse_msg_wheel as u32 != 0
    }

    /// Get wheel delta
    pub fn wheel(&self) -> i32 {
        self.msg.wheel
    }
}
