use xege::*;

fn main() {
    let mut xege = initgraph(640, 480, Init::Default).unwrap();
    xege.setfillcolor(color::RED);
    Draw::fillrect(&mut xege, 10, 10, 100, 100);
    xege.setwritemode(|pen, dst| pen & !dst);
    xege.setfillcolor(color::YELLOW);
    Draw::fillrect(&mut xege, 50, 50, 140, 140);
    loop {}
}
