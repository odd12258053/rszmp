use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};
use uuid::Uuid;

pub fn client(context: Context, addr: &str) {
    let mut socket = context.socket(SocketType::REQ);
    let client_id = Uuid::new_v4();
    socket.set_routing_id(client_id.as_bytes()).unwrap();
    socket.connect(addr).unwrap();

    println!("Send Message");
    let mut msg = Message::from("Hello");
    socket.send_msg(&mut msg, SendFlag::new()).unwrap();
    println!("Sent");

    let mut msg = Message::new();
    let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
    println!("Received {:?}, {}", msg.as_str(), res);
    println!("finished!!");
}
