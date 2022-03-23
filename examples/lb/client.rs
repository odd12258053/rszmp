use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};

pub fn client(context: Context, addr: &str) {
    let mut socket = context.socket(SocketType::REQ);
    socket.set_routing_id("client".as_bytes());
    let rc = socket.connect(addr);
    assert_eq!(rc, 0);

    println!("Send Message");
    let mut msg = Message::from("Hello");
    socket.send_msg(&mut msg, SendFlag::new());
    println!("Sent");

    let mut msg = Message::new();
    let res = socket.recv_msg(&mut msg, RecvFlag::new());
    println!("Received {:?}, {}", msg.as_str(), res);
    println!("finished!!");
}
