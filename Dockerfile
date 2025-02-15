# Stage 1: Build the application
FROM rust:1.84.1-bookworm AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the working directory
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a lightweight image with the binary
FROM debian:bookworm-slim AS runtime

# Install necessary dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -d /home/imapuser imapuser

# Set the working directory inside the container
WORKDIR /home/imapuser

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/imap-attachment-daemon .

# Make the binary executable
RUN chmod +x imap-attachment-daemon

# Copy the .env file
COPY .env .env

# Change ownership of the working directory to the non-root user
RUN chown -R imapuser:imapuser /home/imapuser

# Switch to the non-root user
USER imapuser

# Set the command to run the application
CMD ["./imap-attachment-daemon"]
