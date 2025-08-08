# xege

`EGE`是 Windows 平台下的一个简易 C++ 绘图库，基于 Windows 的 GDI/GDI+ 接口开发。提供了图形绘制、键鼠事件处理等功能。它以易用的API接口而广泛得到初学者的青睐。 `xege-ffi`是 `EGE` 的 Rust FFI 绑定，提供了在 Rust 中调用 `EGE` 的能力。`xege` 是 `xege-ffi` 的 Rust 风格安全封装，使得 Rust 开发者可以更方便地使用 `EGE`。

## 介绍

`EGE`的实现中采用了状态机的机制，所有大多数图形环境类型都不是`Send`或`Sync`的，它们不能跨越线程使用