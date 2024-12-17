FROM rust:bookworm AS builder

RUN apt-get update && apt-get install -y libdbus-1-dev pkg-config build-essential
RUN cargo install cargo-deb

WORKDIR /code

COPY Cargo.lock Cargo.toml LICENSE /code/
COPY agent /code/agent
COPY cache /code/cache
COPY entity /code/entity
COPY server /code/server
COPY storage /code/storage
COPY ui-static /code/ui-static
COPY LICENSE /code/agent/
COPY LICENSE /code/server/

RUN cargo build --release --features collector-atc-sensor \
    && cargo deb -p chezmoi-agent \
    && cargo deb -p chezmoi-server

# RUN ls -lha /code/target/**/* && exit 1

FROM scratch AS output

COPY --from=builder /code/target/release/chezmoi-agent /target/chezmoi-agent
COPY --from=builder /code/target/debian/chezmoi-agent_*.deb /target/
COPY --from=builder /code/target/release/chezmoi-server /target/chezmoi-server
COPY --from=builder /code/target/debian/chezmoi-server_*.deb /target/
