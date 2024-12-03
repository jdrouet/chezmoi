FROM --platform=$BUILDPLATFORM rust:1-bookworm AS server-vendor

ENV USER=root

WORKDIR /code
RUN cargo init --lib --name tekitoi-agent /code/agent
RUN cargo init --lib --name tekitoi-client /code/client
RUN cargo init --lib --name tekitoi-database /code/database
RUN cargo init --lib --name tekitoi-helper /code/helper
RUN cargo init --bin --name tekitoi-server /code/server
COPY Cargo.lock Cargo.toml /code/
COPY agent/Cargo.toml /code/agent/Cargo.toml
COPY client/Cargo.toml /code/client/Cargo.toml
COPY database/Cargo.toml /code/database/Cargo.toml
COPY helper/Cargo.toml /code/helper/Cargo.toml
COPY server/Cargo.toml /code/server/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    mkdir -p /code/.cargo \
    && cargo vendor >> /code/.cargo/config.toml

FROM rust:1-bookworm AS server-builder

RUN apt-get update && apt install -y libdbus-1-dev pkg-config

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY agent/Cargo.toml /code/agent/Cargo.toml
COPY agent/src /code/agent/src
COPY client/Cargo.toml /code/client/Cargo.toml
COPY client/src /code/client/src
COPY database/Cargo.toml /code/database/Cargo.toml
COPY database/migrations /code/database/migrations
COPY database/src /code/database/src
COPY helper/Cargo.toml /code/helper/Cargo.toml
COPY helper/src /code/helper/src
COPY server/Cargo.toml /code/server/Cargo.toml
COPY server/src /code/server/src
COPY --from=server-vendor /code/.cargo /code/.cargo
COPY --from=server-vendor /code/vendor /code/vendor

RUN --mount=type=cache,target=/code/target/release/deps,sharing=locked \
    --mount=type=cache,target=/code/target/release/build,sharing=locked \
    --mount=type=cache,target=/code/target/release/incremental,sharing=locked \
    cargo build --release --offline --package chezmoi-server

RUN strip /code/target/release/chezmoi-server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y libdbus-1-3 \
    && rm -rf /var/lig/apt/lists

ENV HOST=0.0.0.0
ENV PORT=3000

ENV ASSETS_PATH=/etc/chezmoi/assets

COPY client/assets /etc/chezmoi/assets
COPY --from=server-builder /code/target/release/chezmoi-server /bin/chezmoi-server

EXPOSE 3000

ENTRYPOINT [ "/bin/chezmoi-server" ]
