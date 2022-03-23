mod client;
mod server;

use rszmp::{Context, Version};
use std::env;

fn get_addr(args: &mut env::Args) -> String {
    let mut host = None;
    let mut port = None;
    while let Some(arg) = args.next() {
        if arg == "--host" {
            host = Some(args.next().unwrap());
        } else if arg == "--port" {
            port = Some(args.next().unwrap());
        }
    }
    format!(
        "tcp://{}:{}",
        host.unwrap_or("127.0.0.1".to_string()),
        port.unwrap_or("5555".to_string())
    )
}

fn main() {
    let mut args = env::args();
    // Skip execution file.
    args.next();

    let version = Version::new();
    println!(
        "Version: {}.{}.{}",
        version.major, version.minor, version.patch
    );
    let context = Context::new();
    match args.next() {
        Some(arg) if arg == "server" => {
            let addr = get_addr(&mut args);
            server::server(context, addr.as_str())
        }
        Some(arg) if arg == "client" => {
            let addr = get_addr(&mut args);
            client::client(context, addr.as_str())
        }
        _ => panic!("Please set a value for either `server` or `client`."),
    }
}
