# ============================================
# USER SERVICE DOCKERFILE
# Multi-stage build for smaller image size
# ============================================

# Stage 1: Build
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests first (for better caching)
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    mkdir -p src/handlers && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub mod get_user; pub mod get_users; pub mod create_user; pub mod update_user; pub mod delete_user;" > src/handlers/mod.rs && \
    touch src/handlers/get_user.rs src/handlers/get_users.rs src/handlers/create_user.rs src/handlers/update_user.rs src/handlers/delete_user.rs && \
    cargo build --release || true

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/user-service /app/user-service

# Change ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/ || exit 1

# Run the application
CMD ["./user-service"]