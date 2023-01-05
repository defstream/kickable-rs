VERSION 0.6
FROM rust:1.66
WORKDIR /kickable

ARG ORG = defstream

build:
    COPY --dir src Cargo.lock Cargo.toml examples Makefile .
    RUN make build
    SAVE ARTIFACT target/release/kickable kickable
    SAVE ARTIFACT target/release/axum axum
    SAVE ARTIFACT target/release/gotham gotham
    SAVE ARTIFACT target/release/graphul graphul
    SAVE ARTIFACT target/release/poem poem
    SAVE ARTIFACT target/release/rocket rocket
    SAVE ARTIFACT target/release/rouille rouille
    SAVE ARTIFACT target/release/salvo salvo
    SAVE ARTIFACT target/release/trillium trillium
    SAVE ARTIFACT target/release/viz viz
    SAVE ARTIFACT target/release/warp warp

service:
    ARG port=31337
    FROM debian:buster-slim
    EXPOSE $port

wrk:
    FROM debian:buster-slim
    RUN apt-get update
    RUN apt-get install build-essential libssl-dev git unzip -y
    RUN git clone https://github.com/wg/wrk.git wrk
    WORKDIR ./wrk
    RUN make
    RUN cp wrk /usr/local/bin
    ENTRYPOINT ["/usr/local/bin/wrk"]

kickable:
    FROM debian:buster-slim
    COPY +build/kickable /usr/local/bin/kickable
    CMD ["/usr/local/bin/kickable"]
    SAVE IMAGE --push defstream/kickable:latest

axum:
    FROM +service
    COPY +build/axum /usr/local/bin/axum
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push $ORG/kickable-axum:latest

gotham:
    FROM +service
    COPY +build/gotham /usr/local/bin/gotham
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push $ORG/kickable-gotham:latest

graphul:
    FROM +service
    COPY +build/graphul /usr/local/bin/graphul
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push $ORG/kickable-graphul:latest

poem:
    FROM +service
    COPY +build/poem /usr/local/bin/poem
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push $ORG/kickable-poem:latest

rocket:
    FROM +service
    COPY +build/rocket /usr/local/bin/rocket
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push $ORG/kickable-rocket:latest

rouille:
    FROM +service
    COPY +build/rouille /usr/local/bin/rouille
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push $ORG/kickable-rouille:latest

salvo:
    FROM +service
    COPY +build/salvo /usr/local/bin/salvo
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/salvo"]
    SAVE IMAGE --push $ORG/kickable-salvo:latest

trillium:
    FROM +service
    COPY +build/trillium /usr/local/bin/trillium
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/trillium"]
    SAVE IMAGE --push $ORG/kickable-trillium:latest

viz:
    FROM +service
    COPY +build/viz /usr/local/bin/viz
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push $ORG/kickable-viz:latest

warp:
    FROM +service
    COPY +build/warp /usr/local/bin/warp
    EXPOSE 31337
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push $ORG/kickable-warp:latest