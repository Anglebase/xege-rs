use xege::*;

fn main() {
    let mut xege = initgraph(640, 480, Init::NoForceExit).unwrap();
    xege.set_caption("支持 Unicode 字符");
    while xege.is_run() {}
}
