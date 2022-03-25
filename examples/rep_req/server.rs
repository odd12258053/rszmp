use std::{thread, time};

use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};

pub fn server(context: Context, addr: &str) {
    let socket = context.socket(SocketType::REP);
    socket.bind(addr).unwrap();
    loop {
        println!("Wait");
        let mut msg = Message::new();
        let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
        println!("Received Hello: {:?}, {}", msg.as_str(), res);
        thread::sleep(time::Duration::from_millis(100));
        let mut msg = Message::from("World");
        socket.send_msg(&mut msg, SendFlag::new()).unwrap();
    }
}
