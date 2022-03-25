use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};
use std::env;
use uuid::Uuid;

fn client() {
    let context = Context::new();
    let mut socket = context.socket(SocketType::REQ);
    let client_id = Uuid::new_v4();
    socket.set_routing_id(client_id.as_bytes());
    let rc = socket.connect("ipc:///tmp/frontend.ipc");
    assert_eq!(rc, 0);

    println!("Send Message");
    let mut msg = Message::from(format!("{}", client_id).as_str());
    socket.send_msg(&mut msg, SendFlag::new());

    println!("Wait");
    let mut msg = Message::new();
    let res = socket.recv_msg(&mut msg, RecvFlag::new());
    println!("Received Hello: {:?}, {}", msg.as_str(), res);
}

fn worker() {
    let context = Context::new();
    let mut socket = context.socket(SocketType::REP);
    let client_id = Uuid::new_v4();
    socket.set_routing_id(client_id.as_bytes());
    let rc = socket.connect("ipc:///tmp/backend.ipc");
    assert_eq!(rc, 0);
    loop {
        println!("Wait");
        let mut msg = Message::new();
        let res = socket.recv_msg(&mut msg, RecvFlag::new());
        println!("Received Hello: {:?}, {}", msg.as_str(), res);
        println!("Send Message");
        let mut msg = Message::from(format!("done: {}", client_id).as_str());
        socket.send_msg(&mut msg, SendFlag::new());
    }
}

fn proxy() {
    let context = Context::new();
    let frontend = context.socket(SocketType::ROUTER);
    let rc = frontend.bind("ipc:///tmp/frontend.ipc");
    assert_eq!(rc, 0);
    let backend = context.socket(SocketType::DEALER);
    let rc = backend.bind("ipc:///tmp/backend.ipc");
    assert_eq!(rc, 0);
    rszmp::proxy(&frontend, &backend);
}

fn main() {
    let mut args = env::args();
    // Skip execution file.
    args.next();

    match args.next() {
        Some(arg) if arg == "worker" => worker(),
        Some(arg) if arg == "client" => client(),
        Some(arg) if arg == "proxy" => proxy(),
        _ => panic!("Please set a value for either `worker`, `client` or `proxy`."),
    }
}
