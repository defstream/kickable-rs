apt-get update -y
dpkg --add-architecture arm64
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler zip gcc-x86-64-linux-gnu mingw-w64
rustup target add x86_64-pc-windows-gnu
