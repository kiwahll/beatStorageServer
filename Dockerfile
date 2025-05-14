FROM rust:latest AS builder
WORKDIR /usr/src/beatStorageServer
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/beatStorageServer/target/release/beatStorageServer /usr/local/bin/beatStorageServer
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/usr/local/bin/beatStorageServer"]
