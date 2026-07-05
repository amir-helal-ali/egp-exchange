FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /app/target/release/egp-exchange /usr/local/bin/egp-exchange

EXPOSE 3000

CMD ["egp-exchange"]
