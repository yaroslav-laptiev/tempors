FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/tempors /usr/local/bin/tempors

ENV RUST_LOG=info

CMD ["tempors"]
