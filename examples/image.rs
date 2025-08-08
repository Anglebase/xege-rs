use xege::*;

fn main() {
    let img = Image::from_file("examples/demo.png").unwrap();

    let mut xege = initgraph(1000, 800, Init::Default).unwrap();

    // Get red channel
    xege.setfillcolor(Color::rgb(255, 0, 0));
    xege.putimage(0, 0, &img, |pen, src, _dst| src & pen);

    xege.window.flush();
    loop {}
}
