use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};
use uuid::Uuid;

pub fn worker(context: Context, addr: &str) {
    let mut socket = context.socket(SocketType::REQ);
    let worker_id = Uuid::new_v4();
    socket.set_routing_id(worker_id.as_bytes()).unwrap();
    socket.connect(addr).unwrap();
    socket.send("READY".as_bytes(), SendFlag::new()).unwrap();
    loop {
        let mut msg = Message::new();
        let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
        let client_id = msg.as_bytes().unwrap().to_vec();
        println!("client_id: {:?}, {}", client_id, res);
        let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
        let empty = msg.as_bytes();
        println!("empty: {:?}, {}", empty, res);
        let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
        let message = msg.as_bytes();
        println!("message: {:?}, {}", message, res);

        socket
            .send(client_id.as_slice(), SendFlag::SNDMORE())
            .unwrap();
        socket.send(&[], SendFlag::SNDMORE()).unwrap();
        socket.send("OK".as_bytes(), SendFlag::new()).unwrap();
    }
}
