# install rust and dependencies
apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends protobuf-compiler curl ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
export PATH=$HOME/.cargo/bin:$PATH
rustup default stable
