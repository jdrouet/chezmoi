#!/bin/bash

docker build -f script/debian.Dockerfile --target output --output type=local,dest=$(pwd) .
