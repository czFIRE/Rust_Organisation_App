FROM rust:latest

RUN cargo install sqlx-cli

# temporary rust project
# to create another layer for fetching dependencies
RUN cargo new /app

WORKDIR /app

# copy important files
COPY Cargo.lock .
COPY Cargo.toml .

# fetch all dependencies
# and remove temporary rust project
RUN touch src/lib.rs && \
    cargo fetch && \
    rm -rf src

# copy source code
COPY .env.docker .env
COPY . .

CMD sqlx migrate run && cargo run --release
