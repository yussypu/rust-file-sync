use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Please specify mode: 'server' or 'client'");
        return;
    }

    match args[1].as_str() {
        "server" => {
            println!("🚀 Starting server...");
            let _ = rust_file_sync::server::start_server().await; // Ignore the result
        }
        "client" => {
            println!("🚀 Starting client...");
            let _ = rust_file_sync::client::start_client().await; // Ignore the result
        }
        _ => {
            eprintln!("❌ Invalid mode '{}'. Use 'server' or 'client'.", args[1]);
        }
    }
}
