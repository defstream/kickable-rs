#!/bin/bash

dpkg --add-architecture arm64
dpkg --add-architecture amd64
apt-get update -y
DEBIAN_FRONTEND=noninteractive apt-get install -y protobuf-compiler zip gcc-mingw-w64-x86-64 mingw-w64 curl
rustup target add x86_64-pc-windows-gnu

if [ $(dpkg --print-architecture) = 'arm64' ]; then
  DEBIAN_FRONTEND=noninteractive apt-get install -y gcc-x86-64-linux-gnu
fi;
