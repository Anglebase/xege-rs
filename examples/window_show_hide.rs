use std::{thread::sleep, time::Duration};

use xege::*;

fn main() {
    let mut xege = initgraph(640, 480, Init::Hide | Init::NoForceExit).unwrap();
    sleep(Duration::from_secs(1));
    xege.window.show();
    while xege.is_run() {}
}
