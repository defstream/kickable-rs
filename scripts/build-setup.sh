apt-get update -y
dpkg --add-architecture arm64
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler zip gcc-x86-64-linux-gnu mingw-w64 qemu-system binfmt-support qemu-user-static docker
rustup target add x86_64-pc-windows-gnu
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker stop earthly-buildkitd || true
