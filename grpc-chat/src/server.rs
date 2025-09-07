use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("chat");
}

use pb::chat_service_server::{ChatService, ChatServiceServer};
use pb::ChatMessage;

#[derive(Debug, Default)]
pub struct ChatServer {}

type ResponseStream =
    Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl ChatService for ChatServer {
    type ChatMessageStreamingStream = ResponseStream;

    async fn chat_message_streaming(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatMessageStreamingStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.message().await.transpose() {
                match result {
                    Ok(item) => {
                        println!("📩 Received from {}: {}", item.from, item.message);
                        if tx
                            .send(Ok(ChatMessage {
                                from: "Server".to_string(),
                                message: format!("Echo: {}", item.message),
                            }))
                            .await
                            .is_err()
                        {
                            eprintln!("❌ Failed to send back to client");
                            break;
                        }
                    }
                    Err(err) => {
                        eprintln!("❌ Error receiving: {:?}", err);
                        break;
                    }
                }
            }
            println!("✅ Client stream closed");
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::ChatMessageStreamingStream))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = ChatServer::default();

    println!("🚀 Server listening on {}", addr);

    Server::builder()
        .add_service(ChatServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
