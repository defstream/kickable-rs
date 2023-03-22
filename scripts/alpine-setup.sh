apk add protobuf protoc wrk wget docker
wget https://github.com/earthly/earthly/releases/download/v0.7.2/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
service docker start
/usr/local/bin/earthly bootstrap
