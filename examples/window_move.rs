use std::{thread::sleep, time::Duration};
use xege::*;

fn main() {
    let mut xege = initgraph(640, 480, Init::Default).unwrap();
    sleep(Duration::from_secs(3));
    xege.window.move_to(500, 0, true);
    sleep(Duration::from_secs(3));
    xege.closegraph();
}
