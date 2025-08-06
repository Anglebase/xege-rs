use xege::*;

fn main() {
    let xege = initgraph(640, 480, Init::NoForceExit).unwrap();
    while xege.is_run() {
        println!("XEGE is running.");
    }
    println!("XEGE is stopped.");
}
