use crate::XEGE;
use std::ptr::null;
use xege_ffi::*;

/// Console context
#[derive(Debug)]
#[allow(dead_code)]
pub struct Console(*const ());

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { ege_close_console() };
    }
}

impl Console {
    /// close console
    pub fn close(self) {}

    /// show console
    pub fn show(&mut self) -> Result<(), ()> {
        if unsafe { ege_show_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    /// hide console
    pub fn hide(&mut self) -> Result<(), ()> {
        if unsafe { ege_hide_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    /// clear console
    pub fn clear(&mut self) -> Result<(), ()> {
        if unsafe { ege_clear_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Read a character from the console
    pub fn getch(&self) -> Option<char> {
        let ch = unsafe { ege_getch_console() } as u32;
        char::from_u32(ch)
    }
}

impl XEGE {
    /// Create and initialize console
    pub fn initconsole(&mut self) -> Result<Console, ()> {
        if unsafe { ege_init_console() } {
            Ok(Console(null()))
        } else {
            Err(())
        }
    }
}
