use std::{thread::sleep, time::Duration};
use xege::*;

fn main() {
    let xege = initgraph(640, 480, Init::Default).unwrap();
    sleep(Duration::from_secs(5));
    xege.closegraph();
}
