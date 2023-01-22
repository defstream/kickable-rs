apt-get update -y
dpkg --add-architecture arm64
wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends zip binfmt-support qemu-user-static docker
rustup target add x86_64-pc-windows-gnu
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker stop earthly-buildkitd || true
