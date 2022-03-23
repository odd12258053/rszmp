use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};
use uuid::Uuid;

pub fn worker(context: Context, addr: &str) {
    let mut socket = context.socket(SocketType::REQ);
    let worker_id = Uuid::new_v4();
    socket.set_routing_id(worker_id.as_bytes());
    let rc = socket.connect(addr);
    assert_eq!(rc, 0);
    socket.send("READY".as_bytes(), SendFlag::new());
    loop {
        let mut msg = Message::new();
        let res = socket.recv_msg(&mut msg, RecvFlag::new());
        let client_id = msg.as_bytes().unwrap().to_vec();
        println!("client_id: {:?}, {}", client_id, res);
        let res = socket.recv_msg(&mut msg, RecvFlag::new());
        let empty = msg.as_bytes();
        println!("empty: {:?}, {}", empty, res);
        let res = socket.recv_msg(&mut msg, RecvFlag::new());
        let message = msg.as_bytes();
        println!("message: {:?}, {}", message, res);

        socket.send(client_id.as_slice(), SendFlag::SNDMORE());
        socket.send(&[], SendFlag::SNDMORE());
        socket.send("OK".as_bytes(), SendFlag::new());
    }
}
