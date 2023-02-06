FROM rust:1-alpine as builder

WORKDIR /app/

COPY src/ src/
COPY dist/ dist/
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo run

FROM busybox:latest

WORKDIR /root

COPY --from=builder /app/dist dist

COPY httpd.conf dist/

CMD ["busybox", "httpd", "-f", "-v", "-p", "3000", "-h", "dist", "-c", "httpd.conf"]