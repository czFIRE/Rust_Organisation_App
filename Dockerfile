FROM rust:latest

RUN cargo install sqlx-cli

# create temporary rust project
RUN cargo new /app

WORKDIR /app

# copy important files
COPY Cargo.lock .
COPY Cargo.toml .

# download dependencies
RUN cargo fetch

# remove temporary rust project
RUN rm -rf src

# what a hack
COPY . .

# what a genius
CMD sqlx migrate run && cargo run
