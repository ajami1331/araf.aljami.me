FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app/

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
COPY config.prod.toml config.toml

RUN cargo run --release

FROM busybox:latest

WORKDIR /root

COPY --from=builder /app/dist dist

COPY httpd.conf dist/

CMD ["busybox", "httpd", "-f", "-v", "-p", "3000", "-h", "dist", "-c", "httpd.conf"]