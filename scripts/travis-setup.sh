# install rust and protobuf
apt-get update -y
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
rustup default stable
