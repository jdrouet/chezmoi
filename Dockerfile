FROM --platform=$BUILDPLATFORM rust:1-alpine AS server-vendor

ENV USER=root

WORKDIR /code
RUN cargo init --bin --name tekitoi-server /code/server
COPY Cargo.lock Cargo.toml /code/
COPY server/Cargo.toml /code/server/Cargo.toml

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
    --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
    mkdir -p /code/.cargo \
    && cargo vendor >> /code/.cargo/config.toml

FROM rust:1-alpine AS server-builder

RUN apk add --no-cache musl-dev

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY server/Cargo.toml /code/server/Cargo.toml
COPY server/src /code/server/src
COPY --from=server-vendor /code/.cargo /code/.cargo
COPY --from=server-vendor /code/vendor /code/vendor

RUN --mount=type=cache,target=/code/target/release/deps,sharing=locked \
    --mount=type=cache,target=/code/target/release/build,sharing=locked \
    --mount=type=cache,target=/code/target/release/incremental,sharing=locked \
    cargo build --release --offline --package chezmoi-server

RUN strip /code/target/release/chezmoi-server

FROM alpine

ENV HOST=0.0.0.0
ENV PORT=3000

COPY --from=server-builder /code/target/release/chezmoi-server /bin/chezmoi-server

EXPOSE 3000

ENTRYPOINT [ "/bin/chezmoi-server" ]
