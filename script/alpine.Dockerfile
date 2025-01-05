ARG BASE_IMAGE=alpine
FROM $BASE_IMAGE AS builder

RUN apk add --no-cache rust cargo dbus-dev alpine-sdk

WORKDIR /code

COPY Cargo.lock Cargo.toml readme.md /code/
COPY agent /code/agent
COPY entity /code/entity
COPY sensor-prelude /code/sensor-prelude
COPY sensor-system /code/sensor-system
COPY sensor-xiaomi-atc /code/sensor-xiaomi-atc
COPY sensor-xiaomi-miflora /code/sensor-xiaomi-miflora
COPY server /code/server
COPY storage /code/storage
COPY web-prelude /code/web-prelude

# RUN cargo build --locked --release

RUN apk update \
    && abuild-keygen --append -n \
    && cp /root/.abuild/*.rsa.pub /etc/apk/keys/
RUN mv /code/agent/APKBUILD /code/APKBUILD \
    && abuild -F -r -P /code/target/

FROM scratch AS output

ARG TARGET_ARCH
# COPY --from=builder /code/target/release/chezmoi-agent /target/$TARGET_ARCH/
# COPY --from=builder /code/target/release/chezmoi-server /target/$TARGET_ARCH/
COPY --from=builder /code/target/*/*.apk /target/$TARGET_ARCH/
