# See: https://docs.fl0.com/docs/builds/dockerfile/rust/

# Using the `cargo-chef` image to allow caching of the build dependencies.
FROM lukemathwalker/cargo-chef:latest-rust-1.73.0 AS chef

# Define the work directory.
WORKDIR /app

# Prepare the cache...
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cache the dependencies...
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

# Build the project...
COPY . .
RUN cargo build --release

# Using a new base image to execute the compiled binary.
FROM rust:1.73-slim AS quotes

# Define the environment variable needed for the DB.
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Define the environment variable needed for the Web Server.
ARG PORT
ENV PORT=${PORT}

# Move the compiled binary to the new image $PATH.
COPY --from=builder /app/target/release/quotes /usr/local/bin

# Define the entry point as the compiled binary.
ENTRYPOINT ["/usr/local/bin/quotes"]
