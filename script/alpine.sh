#!/bin/bash

docker build \
    -f script/alpine.Dockerfile \
    --build-arg BASE_IMAGE=alpine \
    --build-arg TARGET_ARCH=$(uname -m) \
    --target output \
    --output type=local,dest=$(pwd) .
