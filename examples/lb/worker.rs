use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};

pub fn worker(context: Context, addr: &str) {
    let mut socket = context.socket(SocketType::REQ);
    socket.set_routing_id("worker".as_bytes());
    let rc = socket.connect(addr);
    assert_eq!(rc, 0);
    socket.send("READY".as_bytes(), SendFlag::new());

    let mut msg = Message::new();
    let res = socket.recv_msg(&mut msg, RecvFlag::new());
    let client_id = msg.as_str().unwrap().as_bytes().to_vec();
    println!("client_id: {:?}, {}", client_id, res);
    let res = socket.recv_msg(&mut msg, RecvFlag::new());
    let empty = msg.as_str();
    println!("empty: {:?}, {}", empty, res);
    let res = socket.recv_msg(&mut msg, RecvFlag::new());
    let message = msg.as_str();
    println!("message: {:?}, {}", message, res);

    socket.send(client_id.as_slice(), SendFlag::new().sndmore());
    socket.send(&[], SendFlag::new().sndmore());
    socket.send("OK".as_bytes(), SendFlag::new());
}
