FROM rust:latest

RUN cargo install sqlx-cli

# create temporary rust project
RUN cargo new /app

WORKDIR /app

# copy important files
COPY Cargo.lock .
COPY Cargo.toml .

# tmp
COPY src/lib.rs src/lib.rs

# download dependencies
RUN cargo fetch

# remove temporary rust project
RUN rm -rf src

COPY . .

CMD sqlx migrate run && cargo run
