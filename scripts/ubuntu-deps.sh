# build tools
apt update -y
apt install build-essential libssl-dev git protobuf-compiler unzip -y

# test tools
git clone https://github.com/wg/wrk.git wrk
cd wrk
make
cp wrk /usr/local/bin