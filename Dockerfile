# Use the official Rust image as the base image
FROM rust:1.84

# Install dependencies for pancurses
RUN apt-get update && apt-get install -y \
    libncurses5-dev \
    libncursesw5-dev

# Create a new directory for the project
WORKDIR /app

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Set the entry point to the compiled binary
CMD ["./target/release/rust_snake"]
