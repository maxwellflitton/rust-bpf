# Use the official Rust image (includes Rust toolchain and Cargo)
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy your Rust source files into the container
COPY . .

# Build the Rust project (release build not necessary for simplicity)
RUN cargo build --release

# Expose the port the TCP server listens on (optional for documentation)
EXPOSE 7878

# Run the server binary (adjust path if needed)
CMD ["./target/release/test-tcp"]
