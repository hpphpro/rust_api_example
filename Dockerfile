FROM rust:latest as builder
ENV NAME=axum_api_example

WORKDIR /usr/src

COPY . .

RUN cargo test && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/target/release/${NAME} /usr/local/bin/${NAME}

CMD ["/usr/local/bin/axum_api_example"]