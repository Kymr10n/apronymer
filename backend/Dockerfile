# Multi-stage build for Rust backend
# Stage 1: Build the application
FROM rust:1.82-slim as builder

# Install system dependencies for compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY wordlist ./wordlist


# Build the application (single build, no complex caching)
RUN cargo build --release && chmod +x /app/target/release/apronymer

# Debug: Check the binary size and make sure it's built correctly
RUN ls -la /app/target/release/ && \
    file /app/target/release/apronymer && \
    /app/target/release/apronymer --version || echo "Binary test failed"

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/apronymer /app/apronymer
COPY wordlist ./wordlist
COPY static ./static
EXPOSE 3000
ENV RUST_LOG=info
ENV HOST=0.0.0.0
USER nobody
CMD ["/app/apronymer"]
ENV PORT=3000
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Add a health check command for better container monitoring
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./apronymer"]
