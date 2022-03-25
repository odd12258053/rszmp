use rszmp::{poll, Context, Message, PollItem, RecvFlag, SendFlag, SocketType};

pub fn load_balancer(context: Context, addr1: &str, addr2: &str) {
    let mut frontend = context.socket(SocketType::ROUTER);
    frontend.bind(addr1).unwrap();
    let mut backend = context.socket(SocketType::ROUTER);
    backend.bind(addr2).unwrap();

    let mut workers = Vec::new();
    loop {
        let mut items = [
            PollItem::from_socket(&mut backend, 1),
            PollItem::from_socket(&mut frontend, 1),
        ];
        let size = if workers.len() > 0 { 2 } else { 1 };
        let rc = poll(&mut items[..size], -1).unwrap();
        println!("rc: {}", rc);
        println!("revents0: {}", items[0].get_revents());
        if items[0].get_revents() & 1 == 1 {
            let mut msg = Message::new();
            let res = backend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let worker_id = msg.as_bytes().unwrap().to_vec();
            println!("worker_id: {:?}, {}", worker_id, res);
            let res = backend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let empty = msg.as_bytes();
            println!("empty: {:?}, {}", empty, res);
            let res = backend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let client_id = msg.as_bytes();
            println!("client_id: {:?}, {}", client_id, res);
            if let Some(b"READY") = client_id {
                workers.push(worker_id);
            } else {
                workers.push(worker_id);
                let client_id = client_id.unwrap().to_vec();
                let res = backend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
                let empty = msg.as_bytes();
                println!("empty: {:?}, {}", empty, res);
                let res = backend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
                let message = msg.as_bytes();
                println!("message: {:?}, {}", message, res);
                frontend
                    .send(client_id.as_slice(), SendFlag::SNDMORE())
                    .unwrap();
                frontend.send(&[], SendFlag::SNDMORE()).unwrap();
                frontend.send(message.unwrap(), SendFlag::new()).unwrap();
            }
        }
        println!("revents1: {}", items[1].get_revents());
        if items[1].get_revents() & 1 == 1 {
            let mut msg = Message::new();
            let res = frontend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let client_id = msg.as_bytes().unwrap().to_vec();
            println!("client_id: {:?}, {}", client_id, res);
            let res = frontend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let empty = msg.as_bytes();
            println!("empty: {:?}, {}", empty, res);
            let res = frontend.recv_msg(&mut msg, RecvFlag::new()).unwrap();
            let message = msg.as_bytes();
            println!("message: {:?}, {}", message, res);

            let worker_id = workers.pop();
            println!("worker: {:?}", worker_id);

            backend
                .send(worker_id.unwrap().as_slice(), SendFlag::SNDMORE())
                .unwrap();
            backend.send(&[], SendFlag::SNDMORE()).unwrap();
            backend
                .send(client_id.as_slice(), SendFlag::SNDMORE())
                .unwrap();
            backend.send(&[], SendFlag::SNDMORE()).unwrap();
            backend.send(message.unwrap(), SendFlag::new()).unwrap();
        }
    }
}
