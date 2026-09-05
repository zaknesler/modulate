# syntax=docker/dockerfile:1

FROM rust:1-alpine AS chef

RUN apk add --no-cache musl-dev build-base perl
RUN cargo install cargo-chef --locked

WORKDIR /build

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release \
    && cp target/release/modulate /build/modulate

FROM gcr.io/distroless/static-debian12

LABEL org.opencontainers.image.source="https://github.com/zaknesler/modulate"

COPY --from=builder /build/modulate /usr/local/bin/modulate

ENV MODULATE_HOME=/data
VOLUME ["/data"]

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/modulate"]
CMD ["start"]
