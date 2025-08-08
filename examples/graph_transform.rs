use xege::{mat::*, *};

fn main() {
    let mut xege = initgraph(1600, 900, Init::NoSysDPI).unwrap();

    xege.setbkcolor(color::WHITE);
    xege.setcolor(color::BLACK);
    xege.clear();
    xege.setfont(Font {
        facename: "Segoe UI Emoji".to_string(),
        ..Default::default()
    });

    HighDraw::outtextxy(&mut xege, 250f32, 250f32, "Unicode 字符 🤣😍😊");

    let mat = translate2([100.0, 100.0].into()) * rotate2(radian(30.0));
    xege.set_transform(mat);

    HighDraw::outtextxy(&mut xege, 0f32, 0f32, "Unicode 字符 🤣😍😊");

    loop {}
}
