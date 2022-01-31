FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1.58.1 as chef
WORKDIR /app


FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
ENV SQLX_OFFLINE=true
# Build project dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build our application
COPY . .
RUN cargo build --locked --release --bin zero2prod


FROM docker.io/library/debian:bullseye-slim AS runtime

WORKDIR /app
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["/usr/local/bin/zero2prod"]

# Install OpenSSL - It's dynamically linked by some deps
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY configuration configuration
COPY --from=builder /app/target/release/zero2prod /usr/local/bin/zero2prod
