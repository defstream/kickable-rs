apk add protobuf protoc wrk wget
wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly
chmod +x /usr/local/bin/earthly
/usr/local/bin/earthly bootstrap