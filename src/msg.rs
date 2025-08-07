use xege_ffi::*;

use crate::{Key, Point};

#[bitmask_enum::bitmask]
pub enum KeyFlags {
    Shift,
    Ctrl,
    First,
}

pub enum KeyMsg {
    Down(Key),
    Up(Key),
    Char(char),
}

pub struct MouseMsg {
    pub(crate) msg: ege_mouse_msg,
}

impl MouseMsg {
    pub fn pos(&self) -> Point {
        Point {
            x: self.msg.x,
            y: self.msg.y,
        }
    }

    pub fn is_move(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_move
    }

    pub fn is_down(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_down
    }

    pub fn is_up(&self) -> bool {
        self.msg.msg == ege_mouse_msg_e_mouse_msg_up
    }

    pub fn is_left(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_left as u32 != 0
    }

    pub fn is_right(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_right as u32 != 0
    }

    pub fn is_middle(&self) -> bool {
        self.msg.flags & ege_mouse_flag_e_mouse_flag_mid as u32 != 0
    }

    pub fn is_wheel(&self) -> bool {
        self.msg.flags & ege_mouse_msg_e_mouse_msg_wheel as u32 != 0
    }

    pub fn wheel(&self) -> i32 {
        self.msg.wheel
    }
}
