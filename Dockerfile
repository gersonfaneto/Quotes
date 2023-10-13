# See: https://docs.fl0.com/docs/builds/dockerfile/rust/

FROM lukemathwalker/cargo-chef:latest-rust-1.73.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM rust:1.73-slim AS quotes
COPY --from=builder /app/target/release/quotes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/quotes"]
