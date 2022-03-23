mod client;
mod lb;
mod worker;

use rszmp::{Context, Version};
use std::env;

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
        Some(arg) if arg == "worker" => worker::worker(context, "ipc:///tmp/backend.ipc"),
        Some(arg) if arg == "client" => client::client(context, "ipc:///tmp/frontend.ipc"),
        Some(arg) if arg == "lb" => {
            lb::load_balancer(context, "ipc:///tmp/frontend.ipc", "ipc:///tmp/backend.ipc")
        }
        _ => panic!("Please set a value for either `server` or `client`."),
    }
}
