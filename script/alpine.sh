#!/bin/bash

docker build -f script/alpine.Dockerfile --target output --output type=local,dest=$(pwd) .
