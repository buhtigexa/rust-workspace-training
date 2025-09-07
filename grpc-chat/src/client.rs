use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Channel, Request};

pub mod pb {
    tonic::include_proto!("chat");
}

use pb::{chat_service_client::ChatServiceClient, ChatMessage};

async fn input() -> String {
    use tokio::io::stdin;
    let mut line = String::new();
    let mut reader = BufReader::new(stdin());
    line.clear();
    reader.read_line(&mut line).await.expect("Failed to read line");
    line.trim().to_string()
}

async fn chat(client: &mut ChatServiceClient<Channel>) {
    let (tx, rx) = mpsc::channel(128);
    let in_stream = ReceiverStream::new(rx);

    // Spawn task to read from stdin
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        loop {
            println!("✏️ Enter your message (or type 'exit'):");
            let user_msg = input().await;
            if user_msg.eq_ignore_ascii_case("exit") {
                break;
            }
            let msg = ChatMessage {
                from: "Client".to_string(),
                message: user_msg,
            };
            if tx_clone.send(msg).await.is_err() {
                eprintln!("❌ Failed to send to server");
                break;
            }
        }
    });

    let response = client
        .chat_message_streaming(Request::new(in_stream))
        .await
        .expect("Stream failed");

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.message().await.transpose() {
        match received {
            Ok(msg) => println!("📥 From {}: {}", msg.from, msg.message),
            Err(err) => {
                eprintln!("❌ Error in response: {:?}", err);
                break;
            }
        }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatServiceClient::connect("http://[::1]:50051").await?;
    chat(&mut client).await;
    Ok(())
}
