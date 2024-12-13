#!/bin/bash

docker build -f script/alpine-arm32v6.Dockerfile --target output --output type=local,dest=$(pwd) .
