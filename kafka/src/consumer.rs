use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer, CommitMode};
use rdkafka::error::KafkaError;
use rdkafka::Message;

fn create_consumer() -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "test-group")
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .set("socket.timeout.ms", "4000")
        .create()
        .expect("Consumer creation failed")
}

pub async fn start() -> Result<(), KafkaError> {
    let consumer = create_consumer();
    consumer.subscribe(&["test"])?; // 👈 mismo topic que producer

    println!("✅ Consumer iniciado. Esperando mensajes en el topic 'test'...");

    loop {
        match consumer.recv().await {
            Err(e) => eprintln!("Kafka error: {}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("⚠️  Mensaje vacío"),
                    Some(Ok(payload)) => println!("📩 Recibido: {}", payload),
                    Some(Err(e)) => println!("❌ Error deserializando payload: {:?}", e),
                }
                consumer.commit_message(&message, CommitMode::Async)?;
            }
        }
    }
}
