FROM rust:1.59.0

WORKDIR /app
RUN apt update && apt install clang lld -y
COPY . .
RUN cargo build --release
ENV APP_ENV production
ENTRYPOINT ["./target/release/holosite"]
