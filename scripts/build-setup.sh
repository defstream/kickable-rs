apt-get update -y
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler zip gcc-mingw-w64-x86-64 mingw-w64 gcc-x86-64-linux-gnu curl
rustup target add x86_64-pc-windows-gnu