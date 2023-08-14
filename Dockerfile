FROM rust:alpine AS builder

WORKDIR /app

COPY . .

RUN apk --no-cache add musl-dev postgresql-dev libpq-dev

RUN rustup target add x86_64-unknown-linux-musl
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN  RUSTFLAGS='-C target-feature=-crt-static' cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

ENV RUST_LOG=info

RUN apk --no-cache add libpq-dev libgcc

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/statty .
RUN chmod +x statty

EXPOSE 8080
#ENV DATABASE_URL

CMD ["./statty"]