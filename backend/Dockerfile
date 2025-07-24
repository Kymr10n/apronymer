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
RUN cargo build --release

# Debug: Check the binary size and make sure it's built correctly
RUN ls -la /app/target/release/ && \
    file /app/target/release/apronymer && \
    /app/target/release/apronymer --version || echo "Binary test failed"

# Stage 2: Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN useradd -r -s /bin/false appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/apronymer /app/apronymer

# Ensure apronymer is executable
RUN chmod +x /app/apronymer

# Copy wordlist directory
COPY --from=builder /app/wordlist /app/wordlist

# Copy static files if they exist
COPY static ./static

# Change ownership to app user
RUN chown -R appuser:appuser /app

# Switch to app user
USER appuser

# Expose port
EXPOSE 3000

# Set environment variables with better defaults for containerized deployment
ENV HOST=0.0.0.0
ENV PORT=3000
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Add a health check command for better container monitoring
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./apronymer"]
