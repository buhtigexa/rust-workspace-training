# Etapa 1: build
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copiamos el proyecto completo
COPY . .

# Compilamos en release
RUN cargo build --release 

# Etapa 2: runtime (liviana)
FROM debian:bullseye-slim

# Necesario para que funcione HTTPS
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copiamos solo el binario desde el builder
COPY --from=builder /usr/src/app/target/release/crud .

EXPOSE 3000

CMD ["./crud"]
