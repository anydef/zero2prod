FROM lukemathwalker/cargo-chef:latest-rust-1.80 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release


FROM debian:bookworm-slim AS runtime

RUN apt update -y \
    && apt install -y --no-install-recommends \
    openssl ca-certificates \
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./zero2prod"]
#