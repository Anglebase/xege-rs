use std::time::UNIX_EPOCH;

use xege::*;

fn main() {
    let mut xege = initgraph(1000, 800, Init::Animation).unwrap();

    while xege.is_run() {
        let time = std::time::SystemTime::now();
        let ms = time.duration_since(UNIX_EPOCH).unwrap().as_millis();
        let p1 = xege.getwidth() as f32 / 2f32;
        let p2 = xege.getheight() as f32 / 2f32;
        let r = 200f32;
        let x = p1 + r * ((ms % 1000000) as f32 / 100f32).cos();
        let y = p2 + r * ((ms % 1000000) as f32 / 100f32).sin();
        
        xege.clear();
        xege.setcolor(Color::new(255, 25, 5, 255));
        HighDraw::line(&mut xege, p1, p2, x, y);
        xege.window.flush();
        println!("ms: {ms}");
    }
}
