mod server;
mod client;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("⚠️  Usage: {} [server|client]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => {
            println!("🚀 Starting gRPC server...");
            server::run().await?;
        }
        "client" => {
            println!("🚀 Starting gRPC client...");
            client::run().await?;
        }
        _ => {
            eprintln!("❌ Invalid option: {}. Use 'server' or 'client'", args[1]);
            std::process::exit(1);
        }
    }

    Ok(())
}
