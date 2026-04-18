# ====================
# Builder
# ====================
FROM rust:1.88-alpine AS builder
WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev build-base

COPY . .

RUN cargo build --release

# ====================
# Runtime environment
# ====================
FROM alpine:latest
WORKDIR /app

RUN apk add --no-cache \
    sqlite \
    ca-certificates

COPY --from=builder /usr/src/app/target/release/mqtt-rest-bridge /app/mqtt-rest-bridge

COPY --from=builder /usr/src/app/config /app/config
COPY --from=builder /usr/src/app/queries /app/queries

CMD ["/app/mqtt-rest-bridge"]