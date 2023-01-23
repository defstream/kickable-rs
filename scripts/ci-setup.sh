apt-get update -y
dpkg --add-architecture arm64 amd64
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends zip binfmt-support qemu-user-static
wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
