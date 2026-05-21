# Stage 1: Build
FROM rust:bookworm AS builder

WORKDIR /app

# Cache des dépendances : on build d'abord un projet vide
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build du vrai code (seul ce layer est invalidé quand src/ change)
COPY src/ src/
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime – même base que rust:bookworm pour compatibilité glibc
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cesizen-api /usr/local/bin/cesizen-api

EXPOSE 8080

CMD ["cesizen-api"]
