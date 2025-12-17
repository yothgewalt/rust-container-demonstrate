FROM rust:1.75-slim as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src

RUN touch src/main.rs && \
    cargo build --release

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /app/target/release/rust-container-demonstrate /app/rust-container-demonstrate

EXPOSE 3000

CMD ["/app/rust-container-demonstrate"]
