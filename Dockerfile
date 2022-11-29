FROM rust:1.64 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl ca-certificates
COPY --from=builder ./target/release/finex-funding-bot ./target/release/finex-funding-bot
CMD ["/target/release/finex-funding-bot"]
