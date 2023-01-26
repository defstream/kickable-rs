apt-get update -y
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler zip gcc-x86-64-linux-gnu mingw-w64 curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
rustup target add x86_64-pc-windows-gnu
