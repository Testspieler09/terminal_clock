FROM rust:1.93.0-slim-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --locked

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/tc /usr/local/bin/tc

RUN useradd -m appuser
USER appuser

ENV TERM=xterm-256color

ENTRYPOINT ["tc"]
