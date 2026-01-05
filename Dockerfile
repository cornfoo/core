# syntax=docker/dockerfile:1

FROM rust:1.90.0-bookworm AS builder
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/build/target,sharing=locked,id=rust-target-core \
    cargo build --release --bin api && \
    cp target/release/api .

FROM debian:bookworm-slim AS runtime
WORKDIR /srv

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false svc
USER svc

COPY --from=builder /build/api ./

CMD ["/srv/api"]