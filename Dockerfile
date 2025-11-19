# Build stage
FROM rust:1.70 as builder

WORKDIR /usr/src/cool-names

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/cool-names/target/release/cool-names /app/cool-names

# Copy word files
COPY adjectives.txt /app/adjectives.txt
COPY nouns.txt /app/nouns.txt

# Expose port
EXPOSE 3002

# Run the binary
CMD ["./cool-names"]
