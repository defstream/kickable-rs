VERSION --global-cache 0.8

ARG --global BUILD_DIR=target/x86_64-unknown-linux-musl/release
ARG --global BUILD_FLAGS=--release --all-features --locked
ARG --global BIN_NAME=kickable
ARG --global DIST_DIR=dist
ARG --global REPOSITORY=defstream
ARG --global PACKAGE_NAME=kickable-rs
ARG --global VERSION=0.0.0
ARG --global LABEL_MAINTAINER=Hector Gray <hector@hectorgray.com>
ARG --global port=31337

benchmark:
    FROM debian:buster-slim
    COPY scripts/benchmark-setup.sh scripts/benchmark.sh .
    RUN ./benchmark-setup.sh
    ENTRYPOINT ["benchmark.sh"]

source:
    FROM kickable/builder:latest@sha256:0ca05e7f4682f9bf7effddc4f998710a8b11a57df9b40ec861ff57e878f6b122

    WORKDIR /usr/src/${PACKAGE_NAME}
    COPY --keep-ts --dir i18n scripts examples proto src .
    COPY --keep-ts kickable.yaml Cargo.lock Cargo.toml Makefile build.rs README.md CHANGELOG.md LICENSE.md .

build:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    CACHE target/release
    RUN make build
    SAVE ARTIFACT $BUILD_DIR/kickable ./kickable
    SAVE ARTIFACT kickable.yaml ./kickable.yaml
    SAVE ARTIFACT $BUILD_DIR/axum ./axum
    SAVE ARTIFACT $BUILD_DIR/gotham ./gotham
    SAVE ARTIFACT $BUILD_DIR/graphul ./graphul
    SAVE ARTIFACT $BUILD_DIR/poem ./poem
    SAVE ARTIFACT $BUILD_DIR/rocket ./rocket
    SAVE ARTIFACT $BUILD_DIR/rouille ./rouille
    SAVE ARTIFACT $BUILD_DIR/tonic-client ./tonic-client
    SAVE ARTIFACT $BUILD_DIR/tonic-server ./tonic-server
    SAVE ARTIFACT $BUILD_DIR/viz ./viz
    SAVE ARTIFACT $BUILD_DIR/warp ./warp

kickable-build:
    FROM scratch
    LABEL description="This is this the builder image that offers cross platform rust compilation for kickable that asks the question... Can you kick it?"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY --platform=linux/amd64 --platform=linux/arm64 (+build/kickable) /usr/local/bin/${BIN_NAME}
    COPY --platform=linux/amd64 --platform=linux/arm64 (+build/kickable.yaml) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/kickable"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}:${VERSION} ${REPOSITORY}/${BIN_NAME}:latest

kickable:
    BUILD --platform=linux/amd64 --platform=linux/arm64 +kickable-build

service:
    FROM scratch
    EXPOSE $port

services:
    BUILD --platform=linux/amd64 --platform=linux/arm64 +axum
    BUILD --platform=linux/amd64 --platform=linux/arm64 +gotham
    BUILD --platform=linux/amd64 --platform=linux/arm64 +graphul
    BUILD --platform=linux/amd64 --platform=linux/arm64 +poem
    BUILD --platform=linux/amd64 --platform=linux/arm64 +rocket
    BUILD --platform=linux/amd64 --platform=linux/arm64 +rouille
    BUILD --platform=linux/amd64 --platform=linux/arm64 +tonic-client
    BUILD --platform=linux/amd64 --platform=linux/arm64 +tonic-server
    BUILD --platform=linux/amd64 --platform=linux/arm64 +viz
    BUILD --platform=linux/amd64 --platform=linux/arm64 +warp

axum:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - axum server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/axum /usr/local/bin/axum
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-axum:${VERSION} ${REPOSITORY}/${BIN_NAME}-axum:latest

gotham:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - gotham server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/gotham /usr/local/bin/gotham
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-gotham:${VERSION} ${REPOSITORY}/${BIN_NAME}-gotham:latest

graphul:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - graphul server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/graphul /usr/local/bin/graphul
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-graphul:${VERSION} ${REPOSITORY}/${BIN_NAME}-graphul:latest

poem:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - poem server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/poem /usr/local/bin/poem
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-poem:${VERSION} ${REPOSITORY}/${BIN_NAME}-poem:latest

rocket:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - rocket server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/rocket /usr/local/bin/rocket
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rocket:${VERSION} ${REPOSITORY}/${BIN_NAME}-rocket:latest

rouille:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - rouille server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/rouille /usr/local/bin/rouille
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rouille:${VERSION} ${REPOSITORY}/${BIN_NAME}-rouille:latest

tonic-client:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - tonic client"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/tonic-client /usr/local/bin/tonic-client
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/tonic-client"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-client:${VERSION} ${REPOSITORY}/${BIN_NAME}-tonic-client:latest

tonic-server:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - tonic server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/tonic-server /usr/local/bin/tonic-server
    COPY +build/${BIN_NAME}.yaml  /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/tonic-server"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-server:${VERSION} ${REPOSITORY}/${BIN_NAME}-tonic-server:latest

viz:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - viz server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/viz /usr/local/bin/viz
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-viz:${VERSION} ${REPOSITORY}/${BIN_NAME}-viz:latest

warp:
    FROM +service
    LABEL description="This is the kickable build image that asks the question... Can you kick it? - warp server"
    LABEL maintainer=${LABEL_MAINTAINER}
    COPY +build/warp /usr/local/bin/warp
    COPY +build/${BIN_NAME}.yaml /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-warp:${VERSION} ${REPOSITORY}/${BIN_NAME}-warp:latest

aarch64-apple-darwin:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/aarch64-apple-darwin
    RUN cargo build ${BUILD_FLAGS} --target aarch64-apple-darwin
    SAVE ARTIFACT target/aarch64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/aarch64-apple-darwin/release/axum ./axum
    SAVE ARTIFACT target/aarch64-apple-darwin/release/gotham ./gotham
    SAVE ARTIFACT target/aarch64-apple-darwin/release/graphul ./graphul
    SAVE ARTIFACT target/aarch64-apple-darwin/release/poem ./poem
    SAVE ARTIFACT target/aarch64-apple-darwin/release/rocket ./rocket
    SAVE ARTIFACT target/aarch64-apple-darwin/release/rouille ./rouille
    SAVE ARTIFACT target/aarch64-apple-darwin/release/tonic-client ./tonic-client
    SAVE ARTIFACT target/aarch64-apple-darwin/release/tonic-server ./tonic-server
    SAVE ARTIFACT target/aarch64-apple-darwin/release/viz ./viz
    SAVE ARTIFACT target/aarch64-apple-darwin/release/warp ./warp
    SAVE ARTIFACT ${BIN_NAME}.yaml ./${BIN_NAME}.yaml

aarch64-unknown-linux-musl:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/aarch64-unknown-linux-musl
    RUN cargo build ${BUILD_FLAGS} --target aarch64-unknown-linux-musl
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/axum ./axum
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/gotham ./gotham
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/graphul ./graphul
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/poem ./poem
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/rocket ./rocket
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/rouille ./rouille
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/tonic-client ./tonic-client
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/tonic-server ./tonic-server
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/viz ./viz
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/warp ./warp
    SAVE ARTIFACT ${BIN_NAME}.yaml ./${BIN_NAME}.yaml

x86-64-apple-darwin:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/x86_64-apple-darwin
    RUN cargo build ${BUILD_FLAGS} --target x86_64-apple-darwin
    SAVE ARTIFACT target/x86_64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/x86_64-apple-darwin/release/axum ./axum
    SAVE ARTIFACT target/x86_64-apple-darwin/release/gotham ./gotham
    SAVE ARTIFACT target/x86_64-apple-darwin/release/graphul ./graphul
    SAVE ARTIFACT target/x86_64-apple-darwin/release/poem ./poem
    SAVE ARTIFACT target/x86_64-apple-darwin/release/rocket ./rocket
    SAVE ARTIFACT target/x86_64-apple-darwin/release/rouille ./rouille
    SAVE ARTIFACT target/x86_64-apple-darwin/release/tonic-client ./tonic-client
    SAVE ARTIFACT target/x86_64-apple-darwin/release/tonic-server ./tonic-server
    SAVE ARTIFACT target/x86_64-apple-darwin/release/viz ./viz
    SAVE ARTIFACT target/x86_64-apple-darwin/release/warp ./warp
    SAVE ARTIFACT ${BIN_NAME}.yaml ./${BIN_NAME}.yaml

x86-64-unknown-linux-musl:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/x86_64-unknown-linux-musl
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    RUN cargo build ${BUILD_FLAGS} --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/axum ./axum
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/gotham ./gotham
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/graphul ./graphul
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/poem ./poem
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/rocket ./rocket
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/rouille ./rouille
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/tonic-client ./tonic-client
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/tonic-server ./tonic-server
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/viz ./viz
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/warp ./warp
    SAVE ARTIFACT ${BIN_NAME}.yaml ./${BIN_NAME}.yaml

x86-64-pc-windows-gnu:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/x86_64-pc-windows-gnu
    ENV RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc'
    RUN cargo build ${BUILD_FLAGS} --target x86_64-pc-windows-gnu
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/${BIN_NAME}.exe ./${BIN_NAME}.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/axum.exe ./axum.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/gotham.exe ./gotham.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/graphul.exe ./graphul.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/poem.exe ./poem.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/rocket.exe ./rocket.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/rouille.exe ./rouille.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/tonic-client.exe ./tonic-client.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/tonic-server.exe ./tonic-server.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/viz.exe ./viz.exe
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/warp.exe ./warp.exe
    SAVE ARTIFACT ${BIN_NAME}.yaml ./${BIN_NAME}.yaml

archive:
    FROM --platform linux/arm64 kickable/builder:latest@sha256:0ca05e7f4682f9bf7effddc4f998710a8b11a57df9b40ec861ff57e878f6b122
    WORKDIR /usr/src/archive/aarch64-apple-darwin
    COPY +aarch64-apple-darwin/*  .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 aarch64-apple-darwin.zip *
    RUN sha256sum aarch64-apple-darwin.zip > aarch64-apple-darwin.zip.sha256
    SAVE ARTIFACT aarch64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip
    SAVE ARTIFACT aarch64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/x86_64-apple-darwin
    COPY +x86-64-apple-darwin/*   .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 x86_64-apple-darwin.zip *
    RUN sha256sum x86_64-apple-darwin.zip > x86_64-apple-darwin.zip.sha256
    SAVE ARTIFACT x86_64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip
    SAVE ARTIFACT x86_64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/aarch64-unknown-linux-musl
    COPY +aarch64-unknown-linux-musl/* .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN tar -czvf aarch64-unknown-linux-musl.tar.gz *
    RUN sha256sum aarch64-unknown-linux-musl.tar.gz > aarch64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-unknown-linux-musl
    COPY +x86-64-unknown-linux-musl/* .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN tar -czvf x86_64-unknown-linux-musl.tar.gz *
    RUN sha256sum x86_64-unknown-linux-musl.tar.gz > x86_64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-pc-windows-gnu
    COPY +x86-64-pc-windows-gnu/* .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 x86_64-pc-windows-gnu.zip *
    RUN sha256sum x86_64-pc-windows-gnu.zip > x86_64-pc-windows-gnu.zip.sha256
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip.sha256

cross:
    FROM kickable/builder:latest@sha256:0ca05e7f4682f9bf7effddc4f998710a8b11a57df9b40ec861ff57e878f6b122

    WORKDIR /usr/src/kickable
    COPY --keep-ts  --keep-ts src src
    COPY --keep-ts  proto proto
    COPY --keep-ts  examples examples
    COPY --keep-ts  scripts scripts
    COPY --keep-ts  i18n i18n
    COPY --keep-ts  kickable.yaml Cargo.lock Cargo.toml Makefile build.rs README.md LICENSE.md CHANGELOG.md ./
    RUN cargo build --release --all-features --locked --target aarch64-apple-darwin
    CACHE target/aarch64-unknown-linux-musl
    RUN cargo build --release --all-features --locked --target aarch64-unknown-linux-musl
    CACHE target/x86_64-apple-darwin
    RUN cargo build --release --all-features --locked --target x86_64-apple-darwin
    CACHE target/x86_64-pc-windows-gnu
    ENV RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc'
    RUN cargo build --release --all-features --locked --target x86_64-pc-windows-gnu
    CACHE target/x86_64-unknown-linux-musl
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    RUN cargo build --release --all-features --locked --target x86_64-unknown-linux-musl

    RUN mkdir -p /usr/src/kickable/dist

    # archive x86_64-pc-windows-gnu
    RUN mkdir -p /usr/src/archive/x86_64-pc-windows-gnu
    RUN mv -f target/x86_64-pc-windows-gnu/release/kickable.exe \
            target/x86_64-pc-windows-gnu/release/axum.exe \
            target/x86_64-pc-windows-gnu/release/gotham.exe \
            target/x86_64-pc-windows-gnu/release/graphul.exe \
            target/x86_64-pc-windows-gnu/release/poem.exe \
            target/x86_64-pc-windows-gnu/release/rocket.exe \
            target/x86_64-pc-windows-gnu/release/rouille.exe \
            target/x86_64-pc-windows-gnu/release/tonic-client.exe \
            target/x86_64-pc-windows-gnu/release/tonic-server.exe \
            target/x86_64-pc-windows-gnu/release/viz.exe \
            target/x86_64-pc-windows-gnu/release/warp.exe \
            /usr/src/archive/x86_64-pc-windows-gnu
    RUN cp kickable.yaml README.md LICENSE.md CHANGELOG.md /usr/src/archive/x86_64-pc-windows-gnu
    WORKDIR /usr/src/kickable/dist
    RUN zip -9 x86_64-pc-windows-gnu.zip /usr/src/archive/x86_64-pc-windows-gnu/*
    RUN sha256sum x86_64-pc-windows-gnu.zip > x86_64-pc-windows-gnu.zip.sha256

    # archive aarch64-apple-darwin
    WORKDIR /usr/src/kickable
    RUN mkdir -p /usr/src/archive/aarch64-apple-darwin
    RUN mv -f target/aarch64-apple-darwin/release/kickable \
            target/aarch64-apple-darwin/release/axum \
            target/aarch64-apple-darwin/release/gotham \
            target/aarch64-apple-darwin/release/graphul \
            target/aarch64-apple-darwin/release/poem \
            target/aarch64-apple-darwin/release/rocket \
            target/aarch64-apple-darwin/release/rouille \
            target/aarch64-apple-darwin/release/tonic-client \
            target/aarch64-apple-darwin/release/tonic-server \
            target/aarch64-apple-darwin/release/viz \
            target/aarch64-apple-darwin/release/warp \
            /usr/src/archive/aarch64-apple-darwin
    RUN cp kickable.yaml README.md LICENSE.md CHANGELOG.md /usr/src/archive/aarch64-apple-darwin
    WORKDIR /usr/src/kickable/dist
    RUN zip -9 aarch64-apple-darwin.zip /usr/src/archive/aarch64-apple-darwin/*
    RUN sha256sum aarch64-apple-darwin.zip > aarch64-apple-darwin.zip.sha256

    # archive x86_64-apple-darwin
    WORKDIR /usr/src/kickable
    RUN mkdir -p /usr/src/archive/x86_64-apple-darwin
    RUN mv -f target/x86_64-apple-darwin/release/kickable \
            target/x86_64-apple-darwin/release/axum \
            target/x86_64-apple-darwin/release/gotham \
            target/x86_64-apple-darwin/release/graphul \
            target/x86_64-apple-darwin/release/poem \
            target/x86_64-apple-darwin/release/rocket \
            target/x86_64-apple-darwin/release/rouille \
            target/x86_64-apple-darwin/release/tonic-client \
            target/x86_64-apple-darwin/release/tonic-server \
            target/x86_64-apple-darwin/release/viz \
            target/x86_64-apple-darwin/release/warp \
            /usr/src/archive/x86_64-apple-darwin
    RUN cp kickable.yaml README.md LICENSE.md CHANGELOG.md /usr/src/archive/x86_64-apple-darwin
    WORKDIR /usr/src/kickable/dist
    RUN zip -9 x86_64-apple-darwin.zip /usr/src/archive/x86_64-apple-darwin/*
    RUN sha256sum x86_64-apple-darwin.zip > x86_64-apple-darwin.zip.sha256

    # archive aarch64-unknown-linux-musl
    WORKDIR /usr/src/kickable
    RUN mkdir -p /usr/src/archive/aarch64-unknown-linux-musl
    RUN mv -f target/aarch64-unknown-linux-musl/release/kickable \
            target/aarch64-unknown-linux-musl/release/axum \
            target/aarch64-unknown-linux-musl/release/gotham \
            target/aarch64-unknown-linux-musl/release/graphul \
            target/aarch64-unknown-linux-musl/release/poem \
            target/aarch64-unknown-linux-musl/release/rocket \
            target/aarch64-unknown-linux-musl/release/rouille \
            target/aarch64-unknown-linux-musl/release/tonic-client \
            target/aarch64-unknown-linux-musl/release/tonic-server \
            target/aarch64-unknown-linux-musl/release/viz \
            target/aarch64-unknown-linux-musl/release/warp \
            /usr/src/archive/aarch64-unknown-linux-musl
    RUN cp kickable.yaml README.md LICENSE.md CHANGELOG.md /usr/src/archive/aarch64-unknown-linux-musl
    WORKDIR /usr/src/kickable/dist
    RUN tar -czvf aarch64-unknown-linux-musl.tar.gz /usr/src/archive/aarch64-unknown-linux-musl/*
    RUN sha256sum aarch64-unknown-linux-musl.tar.gz > aarch64-unknown-linux-musl.tar.gz.sha256

    # archive x86_64-unknown-linux-musl
    WORKDIR /usr/src/kickable
    RUN mkdir -p /usr/src/archive/x86_64-unknown-linux-musl
    RUN mv -f target/x86_64-unknown-linux-musl/release/kickable \
            target/x86_64-unknown-linux-musl/release/axum \
            target/x86_64-unknown-linux-musl/release/gotham \
            target/x86_64-unknown-linux-musl/release/graphul \
            target/x86_64-unknown-linux-musl/release/poem \
            target/x86_64-unknown-linux-musl/release/rocket \
            target/x86_64-unknown-linux-musl/release/rouille \
            target/x86_64-unknown-linux-musl/release/tonic-client \
            target/x86_64-unknown-linux-musl/release/tonic-server \
            target/x86_64-unknown-linux-musl/release/viz \
            target/x86_64-unknown-linux-musl/release/warp \
            /usr/src/archive/x86_64-unknown-linux-musl
    RUN cp kickable.yaml README.md LICENSE.md CHANGELOG.md /usr/src/archive/x86_64-unknown-linux-musl
    WORKDIR /usr/src/kickable/dist
    RUN zip -9 x86_64-unknown-linux-musl.zip /usr/src/archive/x86_64-unknown-linux-musl/*
    RUN sha256sum x86_64-unknown-linux-musl.zip > x86_64-unknown-linux-musl.zip.sha256
    RUN tar -czvf x86_64-unknown-linux-musl.tar.gz /usr/src/archive/aarch64-unknown-linux-musl/*
    RUN sha256sum x86_64-unknown-linux-musl.tar.gz > x86_64-unknown-linux-musl.tar.gz.sha256

    SAVE ARTIFACT x86_64-pc-windows-gnu.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip.sha256
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT x86_64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip
    SAVE ARTIFACT x86_64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip.sha256
    SAVE ARTIFACT aarch64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip
    SAVE ARTIFACT aarch64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip.sha256
