
/// Millisecond delay function
/// 
/// # Parameters
/// * `ms` - The number of milliseconds to delay.
pub fn delay_ms(ms: i32) {
    unsafe { xege_ffi::ege_delay_ms(ms) };
}

/// Delay by frame rate.
/// 
/// # Parameters
/// * `fps` - The frame rate to delay.
pub fn delay_fps(fps: f64) {
    unsafe { xege_ffi::ege_delay_fps2(fps) };
}

/// Delay by jitter frame rate.
/// 
/// # Parameters
/// * `fps` - The jitter frame rate to delay.
pub fn delay_jfps(fps: f64) {
    unsafe { xege_ffi::ege_delay_jfps2(fps) };
}
