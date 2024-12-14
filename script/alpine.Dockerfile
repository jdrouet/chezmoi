FROM alpine AS builder

RUN apk add --no-cache rust cargo dbus-dev

WORKDIR /code

COPY Cargo.lock Cargo.toml /code/
COPY agent /code/agent
COPY cache /code/cache
COPY entity /code/entity
COPY server /code/server
COPY storage /code/storage

RUN cargo build --release

FROM scratch AS output

COPY --from=builder /code/target/release/chezmoi-agent /target/chezmoi-agent
COPY --from=builder /code/target/release/chezmoi-server /target/chezmoi-server
