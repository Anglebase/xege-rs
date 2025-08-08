# xege

[`EGE`](https://xege.org/) is an easy-to-use C++ graphics library for the Windows platform, built upon Windows' GDI/GDI+ interfaces. It provides features like graphics rendering and keyboard/mouse event handling. Known for its user-friendly API, it is widely favored by beginners. [`xege-ffi`](https://crates.io/crates/xege-ffi) offers Rust FFI bindings for `EGE`, enabling the usage of `EGE` in Rust. `xege` is a Rust-style safe wrapper around `xege-ffi`, allowing Rust developers to utilize `EGE` more conveniently.

## Introduction

`EGE` employs a state machine mechanism in its implementation. Consequently, most graphics context types are neither `Send` nor `Sync`, meaning they cannot be used across threads.

`EGE` offers powerful drawing capabilities, enabling you to create a wide variety of graphical applications, whether GUI programs or game engines.

## Example

```rust
use xege::*;

fn main() {
    let mut xege = initgraph(1000, 800, Init::NoSysDPI).unwrap();

    xege.setbkcolor(color::DARKGRAY);
    xege.setcolor(color::BLACK);
    xege.setfillcolor(color::CYAN);
    xege.clear();
    xege.enable_aa(true);

    const TEXTS: [&str; 11] = [
        "万籁停吹奏",
        "支颐听秋水问蜉蝣",
        "既玄冥不可量北斗",
        "又何信相思最温柔",
        "顾盼花发鸿蒙",
        "怦然而梦你与二十八宿皆回眸",
        "系我",
        "彩翼鲸尾红丝天地周",
        "情之所至",
        "此心",
        "逍遥不游",
    ];

    let mut path = Path::new();
    for (i, &s) in TEXTS.iter().enumerate() {
        path.addtext(
            30f32,
            10f32 + i as f32 * 60f32,
            s,
            "仿宋",
            48f32,
            FontStyle::Black,
        );
    }
    path.outline(Option::<mat::Mat3<f32>>::None, 0.0);
    xege.drawpath_at(&path, 16.0, 16.0);
    xege.fillpath_at(&path, 15.0, 15.0);
    loop {}
}
```

# License

`xege` is licensed under the MIT License.