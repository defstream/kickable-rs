VERSION 0.6
FROM rust:1.66
WORKDIR /kickable

build:
    COPY --dir src Cargo.lock Cargo.toml examples Makefile .
    RUN make build
    SAVE ARTIFACT target/release/kickable kickable

docker:
    FROM debian:buster-slim
    COPY +build/kickable /usr/local/bin/kickable
    ENTRYPOINT ["/usr/local/bin/kickable"]
    SAVE IMAGE --push defstream/kickable:latest
