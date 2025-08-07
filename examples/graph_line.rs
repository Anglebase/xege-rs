use xege::*;

fn draw(xege: &mut impl Draw) {
    xege.setcolor(Color::new(255, 255, 0, 255));
    xege.setlinewidth(10f32);
    xege.line(50, 60, 120, 180);
}

fn main() {
    let mut xege = initgraph(640, 480, Init::Default).unwrap();
    draw(&mut xege);
    loop {}
}
