FROM alpine AS base

RUN apk add --no-cache rust cargo dbus-dev

WORKDIR /code

COPY Cargo.lock Cargo.toml readme.md /code/
COPY agent /code/agent
COPY cache /code/cache
COPY entity /code/entity
COPY server /code/server
COPY storage /code/storage

FROM base AS builder

RUN cargo build --release

FROM base AS builder-agent-apk

RUN apk update && apk add --no-cache alpine-sdk
RUN abuild-keygen --append -n && cp /root/.abuild/*.rsa.pub /etc/apk/keys/
RUN mv /code/agent/APKBUILD /code/APKBUILD \
    && abuild -F -r -P /code/target

FROM scratch AS output

COPY --from=builder-agent-apk /root/target/*/*.apk /target/
COPY --from=builder-agent-apk /root/target/*/*.apk /target/
COPY --from=builder /code/target/release/chezmoi-agent /target/chezmoi-agent
COPY --from=builder /code/target/release/chezmoi-server /target/chezmoi-server
