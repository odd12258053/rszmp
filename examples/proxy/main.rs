use rszmp::{Context, Message, RecvFlag, SendFlag, SocketType};
use std::env;
use uuid::Uuid;

fn client() {
    let context = Context::new();
    let mut socket = context.socket(SocketType::REQ);
    let client_id = Uuid::new_v4();
    socket.set_routing_id(client_id.as_bytes()).unwrap();
    socket.connect("ipc:///tmp/frontend.ipc").unwrap();

    println!("Send Message");
    let mut msg = Message::from(format!("{}", client_id).as_str());
    socket.send_msg(&mut msg, SendFlag::new()).unwrap();

    println!("Wait");
    let mut msg = Message::new();
    let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
    println!("Received Hello: {:?}, {}", msg.as_str(), res);
}

fn worker() {
    let context = Context::new();
    let mut socket = context.socket(SocketType::REP);
    let client_id = Uuid::new_v4();
    socket.set_routing_id(client_id.as_bytes()).unwrap();
    socket.connect("ipc:///tmp/backend.ipc").unwrap();
    loop {
        println!("Wait");
        let mut msg = Message::new();
        let res = socket.recv_msg(&mut msg, RecvFlag::new()).unwrap();
        println!("Received Hello: {:?}, {}", msg.as_str(), res);
        println!("Send Message");
        let mut msg = Message::from(format!("done: {}", client_id).as_str());
        socket.send_msg(&mut msg, SendFlag::new()).unwrap();
    }
}

fn proxy() {
    let context = Context::new();
    let frontend = context.socket(SocketType::ROUTER);
    frontend.bind("ipc:///tmp/frontend.ipc").unwrap();
    let backend = context.socket(SocketType::DEALER);
    backend.bind("ipc:///tmp/backend.ipc").unwrap();
    rszmp::proxy(&frontend, &backend).unwrap();
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
