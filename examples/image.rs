use xege::*;

fn main() {
    let mut xege = initgraph(1000, 800, Init::Default).unwrap();

    let img = Image::from_file("examples/demo.png").unwrap();

    // Get red channel
    xege.setfillcolor(Color::new(255, 0, 0, 255));
    xege.putimage(10, 10, &img, |pen, src, _dst| src & pen);
    
    xege.window.flush();
    loop {}
}
