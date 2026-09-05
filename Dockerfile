# syntax=docker/dockerfile:1

FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev build-base perl

WORKDIR /build
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && cp target/release/modulate /build/modulate

FROM gcr.io/distroless/static-debian12

LABEL org.opencontainers.image.source="https://github.com/zaknesler/modulate"

COPY --from=builder /build/modulate /usr/local/bin/modulate

ENV MODULATE_HOME=/data
VOLUME ["/data"]

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/modulate"]
CMD ["start"]
