FROM rust:1-bookworm

RUN apt-get update && apt-get install -y libdbus-1-dev pkg-config
