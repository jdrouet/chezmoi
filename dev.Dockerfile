FROM rust:1-bookworm

RUN rustup component add clippy
RUN cargo install bacon
RUN cargo install just

RUN apt-get update && apt-get install -y libdbus-1-dev pkg-config
