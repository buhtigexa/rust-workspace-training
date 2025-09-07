# Kafka Producer & Consumer in Rust

This project demonstrates a simple **Kafka Producer** and **Consumer** written in Rust using the [`rdkafka`](https://crates.io/crates/rdkafka) library.  
Kafka and Zookeeper run locally using **Docker Compose**.

---

## 🚀 Requirements
- [Rust](https://www.rust-lang.org/) with Cargo
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/)

---

## ⚙️ How to Run

Use two terminals:

- **Terminal 1** → Docker/Kafka commands  
- **Terminal 2** → Rust (Cargo) commands  

---

### Full workflow

```bash
# (Terminal 1) Start Kafka & Zookeeper
docker-compose up -d

# (Terminal 1) Create the topic "test"
docker exec -it kafka kafka-topics \
  --create \
  --topic test \
  --partitions 1 \
  --replication-factor 1 \
  --bootstrap-server localhost:9092

# (Terminal 1) Verify the topic
docker exec -it kafka kafka-topics --list --bootstrap-server localhost:9092

# (Terminal 2) Run the Rust application (producer + consumer)
cargo run

# (Terminal 1 - optional) Consume messages manually
docker exec -it kafka kafka-console-consumer \
  --topic test \
  --from-beginning \
  --bootstrap-server localhost:9092

# (Terminal 1 - optional) Produce messages manually
docker exec -it kafka kafka-console-producer \
  --topic test \
  --broker-list localhost:9092

# (Terminal 1) Stop everything
docker-compose down -v

# (Terminal 1 - optional) Clean up Docker
docker system prune -a
