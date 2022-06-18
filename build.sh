#!/bin/bash

export ECR=710915658486.dkr.ecr.us-east-2.amazonaws.com
export VER=0.1.4

docker buildx build --platform linux/amd64 --tag ${ECR}/path_planning:${VER} . --load
# docker buildx build --platform linux/amd64,linux/arm64/v8 --tag ${ECR}/path_planning:${VER} . --push

mkdir -p ${PWD}/build/path-planning
cp -rfp * ${PWD}/build/path-planning

docker run -v ${PWD}/build/path-planning:/opt/path-planning -it ${ECR}/path_planning:${VER}
#docker run -v ${PWD}/build:/opt/path-planning -it arm64v8/pyo3-builder
