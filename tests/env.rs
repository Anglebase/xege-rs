use std::ptr::null_mut;

use xege::*;

#[test]
fn main() {
    let xege = initgraph(640, 480, Init::Default).unwrap();
    assert_ne!(xege.hwnd(), null_mut());
}
