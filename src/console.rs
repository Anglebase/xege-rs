use crate::XEGE;
use std::ptr::null;
use xege_ffi::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Console(*const ());

impl Drop for Console {
    fn drop(&mut self) {
        unsafe { ege_close_console() };
    }
}

impl Console {
    pub fn close(self) {}

    pub fn show(&mut self) -> Result<(), ()> {
        if unsafe { ege_show_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn hide(&mut self) -> Result<(), ()> {
        if unsafe { ege_hide_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        if unsafe { ege_clear_console() } {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn getch(&self) -> Option<char> {
        let ch = unsafe { ege_getch_console() } as u32;
        char::from_u32(ch)
    }
}

impl XEGE {
    pub fn initconsole(&mut self) -> Result<Console, ()> {
        if unsafe { ege_init_console() } {
            Ok(Console(null()))
        } else {
            Err(())
        }
    }
}
