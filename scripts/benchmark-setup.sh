# build tools
apt update -y
apt install build-essential libssl-dev git unzip -y -o Acquire::Retries=5

# test tools
git clone https://github.com/wg/wrk.git wrk
cd wrk
make
cp wrk /usr/local/bin