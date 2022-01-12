FROM docker.io/library/rust:1.57.0

ENV SQLX_OFFLINE true
WORKDIR /app

COPY . .
RUN cargo build --locked --release

ENTRYPOINT ["./target/release/zero2prod"]
