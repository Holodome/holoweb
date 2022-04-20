FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin holosite

FROM chef as database
COPY .data .data
COPY migrations migrations
COPY .env .env

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel database setup

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates sqlite3 \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/debug/holosite holosite
COPY --from=database /app/.data .data
COPY config config
COPY static static
ENV APP_ENV production
ENTRYPOINT ["./holosite"]

