FROM arm32v6/alpine AS builder

RUN apk add --no-cache rust cargo dbus-dev

WORKDIR /code

COPY Cargo.lock Cargo.toml agent/APKBUILD /code/
COPY agent /code/agent
COPY cache /code/cache
COPY entity /code/entity
COPY server /code/server
COPY storage /code/storage

RUN cargo build --release

RUN apk add --no-cache abuild && abuild build

FROM scratch AS output

COPY --from=builder /code/target/release/chezmoi-agent /target/chezmoi-agent_arm32v6
COPY --from=builder /code/target/release/chezmoi-server /target/chezmoi-server_arm32v6
