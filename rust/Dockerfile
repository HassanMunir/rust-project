# Use the official Rust image as a builder
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/app

# Copy Rust project files from the `rust/` directory
COPY . . 

# Build the application
RUN cargo build --release

# Create a new minimal image to run the application
FROM debian:bullseye-slim

# Install necessary dependencies for running the Rust app (e.g., libc)
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/rust-backend-app /usr/src/app/rust-backend-app

# Expose the port the app will run on
EXPOSE 8020

# Command to run the application
CMD ["./rust-backend-app"]
