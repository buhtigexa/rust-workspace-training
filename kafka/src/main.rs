mod producer;
mod consumer;

#[tokio::main]
async fn main() {
    let producer = producer::create();

    producer::produce(&producer, "Hello, Kafka!".to_string()).await;

    if let Err(e) = consumer::start().await {
        eprintln!("❌ Error en consumer: {:?}", e);
    }
}
