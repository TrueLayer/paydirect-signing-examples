#!/usr/bin/env sh
set -e
docker build -t truelayer-cert-gen -f Dockerfile.openssl .
vol=$(uuidgen)
docker volume create "${vol}"
docker run -v "${vol}:/out" truelayer-cert-gen
cid=$(docker ps -aq -n 1)
rm -rf out || return
mkdir -p out
docker cp ${cid}:/out/ .
