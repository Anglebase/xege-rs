use std::{thread::sleep, time::Duration};
use xege::*;

// BUG: The window is not resized to the desired size.
fn main() {
    let mut xege = initgraph(640, 480, Init::Default).unwrap();
    sleep(Duration::from_secs(3));
    xege.window.resize(1600, 900);
    println!("Resized window to 1600x900");
    sleep(Duration::from_secs(3));
    xege.closegraph();
}
