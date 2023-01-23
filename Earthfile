VERSION 0.6
ARG version = 0.0.0
ARG ORG = defstream
ARG BIN_NAME = kickable
ARG GITHUB_REPOSITORY = ghcr.io/defstream/kickable-rs
ARG DOCKER_HUB_REPOSITORY = kickable
ARG PACKAGE_NAME = kickable-rs
ARG DIST_DIR = dist
ARG DIST_FILES = ./README.md ./LICENSE ./CHANGELOG.md
ARG BUILD_DIR = target/x86_64-unknown-linux-musl/release
ARG BUILD_FLAGS = --release --all-features --locked
ARG BUILD_PLATFORMS =

benchmark:
    FROM debian:buster-slim
    COPY scripts/benchmark-setup.sh scripts/benchmark.sh .
    RUN ./benchmark-setup.sh
    ENTRYPOINT ["benchmark.sh"]

builder:
    FROM joseluisq/rust-linux-darwin-builder
    COPY scripts/build-setup.sh .
    RUN ./build-setup.sh

source:
    FROM +builder
    WORKDIR /usr/src/${PACKAGE_NAME}
    COPY --dir scripts examples proto src .
    COPY Cargo.lock Cargo.toml Makefile build.rs README.md CHANGELOG.md LICENSE ./

build:
    FROM +source
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
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

kickable:
    BUILD --platform=linux/amd64 --platform=linux/arm64 +kickable-build

kickable-build:
    FROM scratch
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/${BIN_NAME} /usr/local/bin/kickable
    ENTRYPOINT ["/usr/local/bin/kickable"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}:${VERSION}

service:
    ARG port=31337
    FROM scratch
    EXPOSE $port

axum:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY --platform=linux/amd64 (+build/axum) /usr/local/bin/axum
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-axum:${VERSION}

gotham:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/gotham /usr/local/bin/gotham
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-gotham:${VERSION}

graphul:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/graphul /usr/local/bin/graphul
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-graphul:${VERSION}

poem:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/poem /usr/local/bin/poem
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-poem:${VERSION}

rocket:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/rocket /usr/local/bin/rocket
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rocket:${VERSION}

rouille:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/rouille /usr/local/bin/rouille
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rouille:${VERSION}

salvo:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/salvo /usr/local/bin/salvo
    ENTRYPOINT ["/usr/local/bin/salvo"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-salvo:${VERSION}

tonic-client:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/tonic-client /usr/local/bin/tonic-client
    ENTRYPOINT ["/usr/local/bin/tonic-client"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-client:${VERSION}

tonic-server:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY = ${ORG}
    COPY +build/tonic-server /usr/local/bin/tonic-server
    ENTRYPOINT ["/usr/local/bin/tonic-server"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-server:${VERSION}

trillium:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/trillium /usr/local/bin/trillium
    ENTRYPOINT ["/usr/local/bin/trillium"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-trillium:${VERSION}

viz:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/viz /usr/local/bin/viz
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-viz:${VERSION}

warp:
    FROM +service
    ARG VERSION=latest
    ARG REPOSITORY=${ORG}
    COPY +build/warp /usr/local/bin/warp
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-warp:${VERSION}


aarch64-apple-darwin:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target aarch64-apple-darwin
    SAVE ARTIFACT target/aarch64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/aarch64-apple-darwin/release/axum .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/gotham .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/graphul .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/poem
    SAVE ARTIFACT target/aarch64-apple-darwin/release/rocket .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/rouille .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/salvo .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/tonic-client .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/tonic-server .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/trillium .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/viz .
    SAVE ARTIFACT target/aarch64-apple-darwin/release/warp .

aarch64-unknown-linux-musl:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target aarch64-unknown-linux-musl
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/axum .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/gotham .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/graphul .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/poem
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/rocket .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/rouille .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/salvo .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/tonic-client .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/tonic-server .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/trillium .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/viz .
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/warp .

x86-64-apple-darwin:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target x86_64-apple-darwin
    SAVE ARTIFACT target/x86_64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/x86_64-apple-darwin/release/axum .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/gotham .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/graphul .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/poem
    SAVE ARTIFACT target/x86_64-apple-darwin/release/rocket .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/rouille .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/salvo .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/tonic-client .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/tonic-server .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/trillium .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/viz .
    SAVE ARTIFACT target/x86_64-apple-darwin/release/warp .

x86-64-unknown-linux-musl:
    FROM +source
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    RUN cargo build ${BUILD_FLAGS} --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/axum .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/gotham .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/graphul .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/poem
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/rocket .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/rouille .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/salvo .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/tonic-client .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/tonic-server .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/trillium .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/viz .
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/warp .

x86-64-pc-windows-gnu:
    FROM +source
    ENV RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc'
    RUN cargo build ${BUILD_FLAGS}  --target x86_64-pc-windows-gnu
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/${BIN_NAME}.exe ${BIN_NAME}.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/axum.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/gotham.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/graphul.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/poem.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/rocket.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/rouille.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/salvo.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/tonic-client.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/tonic-server.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/trillium.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/viz.exe .
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/warp.exe .

archive:
    FROM +builder
    ARG VERSION = 0.0.0

    WORKDIR /usr/src/archive/aarch64-apple-darwin
    COPY +aarch64-apple-darwin/* .
    COPY README.md LICENSE CHANGELOG.md  .
    RUN zip -9 aarch64-apple-darwin.zip *
    RUN sha256sum aarch64-apple-darwin.zip > aarch64-apple-darwin.zip.sha256
    SAVE ARTIFACT aarch64-apple-darwin.zip AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip
    SAVE ARTIFACT aarch64-apple-darwin.zip.sha256 AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/x86_64-apple-darwin
    COPY +x86-64-apple-darwin/* .
    COPY README.md LICENSE CHANGELOG.md  .
    RUN zip -9 x86_64-apple-darwin.zip *
    RUN sha256sum x86_64-apple-darwin.zip > x86_64-apple-darwin.zip.sha256
    SAVE ARTIFACT x86_64-apple-darwin.zip AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip
    SAVE ARTIFACT x86_64-apple-darwin.zip.sha256 AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/aarch64-unknown-linux-musl
    COPY +aarch64-unknown-linux-musl/* .
    COPY README.md LICENSE CHANGELOG.md  .
    RUN tar -czvf aarch64-unknown-linux-musl.tar.gz *
    RUN sha256sum aarch64-unknown-linux-musl.tar.gz > aarch64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-unknown-linux-musl
    COPY +x86-64-unknown-linux-musl/* .
    COPY README.md LICENSE CHANGELOG.md  .
    RUN tar -czvf x86_64-unknown-linux-musl.tar.gz *
    RUN sha256sum x86_64-unknown-linux-musl.tar.gz > x86_64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-pc-windows-gnu
    COPY +x86-64-pc-windows-gnu/* .
    COPY README.md LICENSE CHANGELOG.md  .
    RUN zip -9 x86_64-pc-windows-gnu.zip *
    RUN sha256sum x86_64-pc-windows-gnu.zip > x86_64-pc-windows-gnu.zip.sha256
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip.sha256 AS LOCAL ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip.sha256

release:
    FROM +archive --VERSION=${VERSION}
    WORKDIR release
    COPY scripts/release-setup.sh .
    RUN ./release-setup.sh
    RUN ls
    SAVE IMAGE $ORG/${PACKAGE_NAME}-release:${VERSION}
