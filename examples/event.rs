use xege::*;

fn main() {
    let xege = initgraph(640, 480, Init::Default).unwrap();
    loop {
        let kbmsg = xege.window.getmsg();
        if let Some((keymsg, keyflags)) = kbmsg {
            println!("KeyMsg: {:?}, KeyFlags: {:?}", keymsg, keyflags)
        }
        let ms = xege.window.getmouse();
        if let Some(mousemsg) = ms {
            println!("MouseMsg: {:?}", mousemsg)
        }
    }
}
