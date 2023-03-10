apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends zip qemu-system binfmt-support protobuf-compiler qemu-user-static gcc-mingw-w64-x86-64 gcc-x86-64-linux-gnu mingw-w64 iptables
wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker stop earthly-buildkitd || true

