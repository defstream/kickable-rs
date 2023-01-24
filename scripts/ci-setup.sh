apt-get update -y
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends zip qemu-system binfmt-support qemu-user-static x86_64-w64-mingw32-gcc gcc-x86-64-linux-gnu mingw-w64 docker iptables
wget https://download.docker.com/linux/static/stable/x86_64/docker-20.10.9.tgz && tar -xzvf docker-20.10.9.tgz && cp docker/* /usr/bin/ && dockerd
wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker stop earthly-buildkitd || true
