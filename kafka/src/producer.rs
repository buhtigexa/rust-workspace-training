use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rdkafka::util::Timeout;
use std::time::Duration;

pub fn create() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error")
}

pub async fn produce(future_producer: &FutureProducer, msg: String) {
    let record = FutureRecord::to("test") // 👈 mismo topic que consumer
        .payload(&msg)
        .key("test-key");

    let status_delivery = future_producer
        .send(record, Timeout::After(Duration::from_secs(2)))
        .await;

    match status_delivery {
        Ok(delivery) => println!("✅ Sent: {:?}", delivery),
        Err((e, _)) => eprintln!("❌ Error: {:?}", e),
    }
}
