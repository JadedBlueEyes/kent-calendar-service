FROM rust:latest AS builder

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/rust-secure-code/cargo-auditable/releases/download/v0.6.4/cargo-auditable-installer.sh | sh

WORKDIR /app

COPY . .

ENV RUSTFLAGS='-C target-feature=+crt-static'
ENV CARGO_INCREMENTAL=0

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo auditable build --locked --release --target x86_64-unknown-linux-gnu && \
    cp ./target/x86_64-unknown-linux-gnu/release/kent-calendar-service /kent-calendar-service

# serve

FROM scratch

# Import from builder.

WORKDIR /app

# Copy our build
COPY --from=builder /kent-calendar-service ./app 

ENV HOST=0.0.0.0:3000
EXPOSE 3000


CMD ["/app/app"]
