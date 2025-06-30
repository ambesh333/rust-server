# Build Stage
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime Stage with compatible glibc
FROM debian:bookworm-slim

COPY --from=builder /usr/src/app/target/release/rust-web-server /app

EXPOSE 3000
CMD ["/app"]
