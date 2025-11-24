FROM rust:1.81 as builder
WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY web ./web

RUN cargo build --release --bin installer-web --bin install-nothing

FROM debian:bookworm-slim
RUN useradd -m -u 10001 app
WORKDIR /app

# Copy binaries and static assets
COPY --from=builder /app/target/release/installer-web /usr/local/bin/
COPY --from=builder /app/target/release/install-nothing /usr/local/bin/
COPY --from=builder /app/web ./web

ENV RUST_LOG=info
EXPOSE 3000
USER app
CMD ["installer-web"]

