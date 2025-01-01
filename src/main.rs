mod server;
mod client;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: chat_app <server|client>");
        return;
    }

    match args[1].as_str() {
        "server" => server::start_server(),
        "client" => client::start_client(),
        _ => eprintln!("Invalid option. Use 'server' or 'client'."),
    }
}

