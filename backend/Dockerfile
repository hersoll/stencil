# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY .sqlx ./.sqlx
COPY db ./db
COPY macros ./macros
COPY math ./math
COPY problems ./problems
COPY registry ./registry
COPY server ./server
COPY types ./types
COPY typst_writer ./typst_writer
COPY preamble.typ ./preamble.typ

# Build release binary
ENV SQLX_OFFLINE=true
RUN cargo build --release --features docker

# Runtime stage
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    xz-utils \
    fonts-liberation \
    fonts-dejavu \
    && rm -rf /var/lib/apt/lists/*

# Install Typst
RUN curl -fsSL https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz \
    | tar -xJ --strip-components=1 -C /usr/local/bin


WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/stencil /app/stencil

COPY preamble.typ /app/preamble.typ

EXPOSE 3000

CMD ["/app/stencil"]
