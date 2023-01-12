VERSION 0.6

ARG ORG = defstream
ARG BUILD_DIR = target/x86_64-unknown-linux-musl/release

shipyard:
    FROM messense/rust-musl-cross:x86_64-musl
    WORKDIR /usr/local/src/kickable/src

    RUN apt-get update
    RUN apt-get install protobuf-compiler -y

build:
    FROM +shipyard
    COPY --dir src Cargo.lock Cargo.toml examples proto build.rs Makefile .
    RUN make build
    SAVE ARTIFACT $BUILD_DIR/kickable kickable
    SAVE ARTIFACT $BUILD_DIR/axum axum
    SAVE ARTIFACT $BUILD_DIR/gotham gotham
    SAVE ARTIFACT $BUILD_DIR/graphul graphul
    SAVE ARTIFACT $BUILD_DIR/poem poem
    SAVE ARTIFACT $BUILD_DIR/rocket rocket
    SAVE ARTIFACT $BUILD_DIR/rouille rouille
    SAVE ARTIFACT $BUILD_DIR/salvo salvo
    SAVE ARTIFACT $BUILD_DIR/tonic-client tonic-client
    SAVE ARTIFACT $BUILD_DIR/tonic-server tonic-server
    SAVE ARTIFACT $BUILD_DIR/trillium trillium
    SAVE ARTIFACT $BUILD_DIR/viz viz
    SAVE ARTIFACT $BUILD_DIR/warp warp

service:
    ARG port=31337
    FROM scratch
    EXPOSE $port

kickable:
    FROM scratch
    COPY +build/kickable /usr/local/bin/kickable
    CMD ["/usr/local/bin/kickable"]
    SAVE IMAGE --push $ORG/kickable:latest

axum:
    FROM +service
    COPY +build/axum /usr/local/bin/axum
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push $ORG/kickable-axum:latest

gotham:
    FROM +service
    COPY +build/gotham /usr/local/bin/gotham
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push $ORG/kickable-gotham:latest

graphul:
    FROM +service
    COPY +build/graphul /usr/local/bin/graphul
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push $ORG/kickable-graphul:latest

poem:
    FROM +service
    COPY +build/poem /usr/local/bin/poem
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push $ORG/kickable-poem:latest

rocket:
    FROM +service
    COPY +build/rocket /usr/local/bin/rocket
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push $ORG/kickable-rocket:latest

rouille:
    FROM +service
    COPY +build/rouille /usr/local/bin/rouille
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push $ORG/kickable-rouille:latest

salvo:
    FROM +service
    COPY +build/salvo /usr/local/bin/salvo
    ENTRYPOINT ["/usr/local/bin/salvo"]
    SAVE IMAGE --push $ORG/kickable-salvo:latest

tonic-client:
    FROM +service
    COPY +build/tonic-client /usr/local/bin/tonic-client
    ENTRYPOINT ["/usr/local/bin/tonic-client"]
    SAVE IMAGE --push $ORG/kickable-tonic-client:latest

tonic-server:
    FROM +service
    COPY +build/tonic-server /usr/local/bin/tonic-server
    ENTRYPOINT ["/usr/local/bin/tonic-server"]
    SAVE IMAGE --push $ORG/kickable-tonic-server:latest

trillium:
    FROM +service
    COPY +build/trillium /usr/local/bin/trillium
    ENTRYPOINT ["/usr/local/bin/trillium"]
    SAVE IMAGE --push $ORG/kickable-trillium:latest

viz:
    FROM +service
    COPY +build/viz /usr/local/bin/viz
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push $ORG/kickable-viz:latest

warp:
    FROM +service
    COPY +build/warp /usr/local/bin/warp
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push $ORG/kickable-warp:latest

wrk:
    FROM debian:buster-slim
    COPY scripts/ubuntu-deps.sh .
    COPY scripts/benchmark.sh .
    RUN ./ubuntu-deps.sh
    ENTRYPOINT ["benchmark.sh"]
