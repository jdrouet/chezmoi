#!/bin/bash

docker build \
    -f script/alpine.Dockerfile \
    --build-arg BASE_IMAGE=arm32v6/alpine \
    --build-arg TARGET_ARCH=arm32v6 \
    --target output \
    --output type=local,dest=$(pwd) .
