# Stage 1: Create a builder image with Rust and Cargo dependencies.
# NOTE: The `rust-musl-builder` base image is used as our `runtime` is an `alpine` image.
FROM clux/muslrust:stable AS chef

# Set the working directory for the image
WORKDIR /app

# NOTE: The `cargo-chef` package allows for the caching of the build dependencies.
RUN cargo install cargo-chef

# Stage 2: Copy the source code and prepare the build recipe.
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build the application using the prepared recipe.
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Copy the source code and build the release version.
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 4: Create the final image for the application.
# NOTE: The `alpine` base image is used, because it's lighter and we don't need the Rust toolchain anymore.
FROM alpine:latest AS runtime

# Set environment variables for the application.
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

ARG PORT
ENV PORT=${PORT}

# Set the working directory for the image
WORKDIR /app

# Copy the built application from the `builder` stage to the final image.
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/quotes /usr/local/bin/

# Set the entry point for the application.
ENTRYPOINT ["/usr/local/bin/quotes"]
