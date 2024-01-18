FROM rust:alpine3.18 as builder

RUN apk add pkgconfig
RUN apk add libressl-dev
RUN apk add musl-dev

WORKDIR /app/

COPY .cargo Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

FROM alpine:3.18 as runner

WORKDIR /app

COPY .env .
COPY --from=builder /app/target/release/uda-results-extractor .

ENTRYPOINT ["/app/uda-results-extractor"]