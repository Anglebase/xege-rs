use std::ptr::null;
use std::{ptr::null_mut, sync::Mutex};
use xege_ffi::*;

use crate::graphics::DrawableDevice;
use crate::window::Window;

/// The initialization options.
#[bitmask_enum::bitmask(i32)]
pub enum Init {
    /// Default initialization mode.
    Default = ege_initmode_flag_INIT_DEFAULT,
    /// Topmost window.
    TopMost = ege_initmode_flag_INIT_TOPMOST,
    /// No border window.
    NoBorder = ege_initmode_flag_INIT_NOBORDER,
    /// Child window.
    Child = ege_initmode_flag_INIT_CHILD,
    /// Manual rendering mode.
    RenderManual = ege_initmode_flag_INIT_RENDERMANUAL,
    /// No force exit when closing window.
    ///
    /// # Note
    /// When exiting, the program is not forced to end, but a flag is set,
    /// which can be determined by `XEGE::is_run()`. When this mode is set,
    /// the window will not close and the program will not exit when the
    /// user performs a window closing operation. At this time, `XEGE::is_run()`
    /// returns false. At the same time, blocking functions such as `getch()`
    /// and `getmouse()` will no longer block. Since the window is not actively
    /// closed, it is necessary to handle the window closure and program exit
    /// in the code in the future.
    NoForceExit = ege_initmode_flag_INIT_NOFORCEEXIT,
    /// Hide the window at the beginning of creation.
    Hide = ege_initmode_flag_INIT_HIDE,
    /// Play the logo animation after creating the window.
    WithLogo = ege_initmode_flag_INIT_WITHLOGO,
    /// Animation mode. Equivalent to `Init::RenderManual | Init::NoForceExit`
    Animation = ege_initmode_flag_INIT_ANIMATION,
}

/// Graphics environment.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XEGE {
    pub window: Window,
}

#[derive(Debug, thiserror::Error)]
pub enum XEGEError {
    #[error("XEGE is already initialized.")]
    Initialized,
}

const INIT_FLAG: Mutex<bool> = Mutex::new(false);

/// Initializes the graphics environment.
///
/// # Arguments
/// * `width` - The width of the window.
/// * `height` - The height of the window.
/// * `mode` - The initialization mode. See `Init` for more information.
///
/// # Errors
///
/// Returns an error if XEGE is already initialized.
pub fn initgraph(width: i32, height: i32, mode: Init) -> Result<XEGE, XEGEError> {
    {
        let flag = INIT_FLAG;
        let mut lock = flag.lock().unwrap();
        if *lock {
            return Err(XEGEError::Initialized);
        }
        *lock = true;
    }
    unsafe {
        ege_initgraph(width, height, mode.bits | ege_initmode_flag_INIT_UNICODE);
    }
    Ok(XEGE {
        window: Window(null_mut()),
    })
}

impl Drop for XEGE {
    fn drop(&mut self) {
        {
            let flag = INIT_FLAG;
            let mut lock = flag.lock().unwrap();
            *lock = false;
        }
        unsafe { ege_closegraph() };
    }
}

impl XEGE {
    /// Closes the graphics environment.
    pub fn closegraph(self) {}

    /// Checks if the graphics environment is running.
    ///
    /// # Return
    /// `true` if the graphics environment is running, `false` otherwise.
    ///
    /// # Note
    /// This function is only valid when `Init::NoForceExit` is present in initialization mode.
    pub fn is_run(&self) -> bool {
        unsafe { ege_is_run() }
    }

    /// Sets the window caption.
    /// 
    /// # Arguments
    /// * `caption` - The caption of the window.
    /// 
    /// # Note
    /// The caption is displayed in the title bar of the window.
    pub fn set_caption(&mut self, caption: &str) {
        let wchar_arr: Vec<u16> = caption.encode_utf16().chain(Some(0)).collect();
        let wchar_ptr = wchar_arr.as_ptr();
        unsafe { ege_setcaption1(wchar_ptr) };
    }

    /// Get the window handle.
    ///
    /// # Return
    /// The window handle.
    ///
    /// # Note
    /// You can manipulate windows through the WIN32 API.
    pub fn hwnd(&self) -> HWND {
        unsafe { ege_getHWnd() }
    }
}

impl DrawableDevice for XEGE {
    fn mut_ptr(&mut self) -> *mut ege_IMAGE {
        null_mut()
    }

    fn const_ptr(&self) -> *const ege_IMAGE {
        null()
    }
}
