apt-get update -y
 DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler zip gcc-x86-64-linux-gnu mingw-w64
 rustup target add x86_64-pc-windows-gnu
