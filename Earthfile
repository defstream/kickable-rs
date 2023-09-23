VERSION 0.7

ARG --global BUILD_DIR=target/x86_64-unknown-linux-musl/release
ARG --global BUILD_FLAGS=--release --all-features --locked
ARG --global BIN_NAME=kickable
ARG --global DIST_DIR=dist
ARG --global REPOSITORY=defstream
ARG --global PACKAGE_NAME=kickable-rs
ARG --global VERSION=0.0.0

benchmark:
    FROM debian:buster-slim
    COPY scripts/benchmark-setup.sh scripts/benchmark.sh .
    RUN ./benchmark-setup.sh
    ENTRYPOINT ["benchmark.sh"]

source:
    ARG PACKAGE_NAME
    FROM kickable/builder:latest@sha256:2c8149ee36a109fc6449f0af6b7588a16a3dcd9067665d1052dfb492dec57d6b
    WORKDIR /usr/src/${PACKAGE_NAME}
    COPY --dir i18n scripts examples proto src .
    COPY kickable.yaml Cargo.lock Cargo.toml Makefile build.rs README.md CHANGELOG.md LICENSE.md .

build:
    ARG BUILD_DIR
    ARG BUILD_FLAGS
    ARG PACKAGE_NAME
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

kickable:
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG PACKAGE_NAME
    ARG REPOSITORY
    ARG VERSION
    BUILD --platform=linux/amd64 +kickable-build --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}

kickable-build:
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG PACKAGE_NAME
    ARG REPOSITORY
    ARG VERSION
    FROM scratch
    COPY --platform=linux/amd64 (+build/kickable --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/${BIN_NAME}
    COPY --platform=linux/amd64 (+build/kickable.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config

    ENTRYPOINT ["/usr/local/bin/kickable"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}:${VERSION} ${REPOSITORY}/${BIN_NAME}:latest

service:
    ARG port=31337
    FROM scratch
    EXPOSE $port

services:
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG PACKAGE_NAME
    ARG REPOSITORY
    ARG VERSION
    BUILD --platform=linux/amd64 --platform=linux/arm64 +axum --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +gotham --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +graphul --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +poem --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +rocket --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +rouille --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} -REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +tonic-client --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +tonic-server --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +viz --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}
    BUILD --platform=linux/amd64 --platform=linux/arm64 +warp --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}

axum:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG PACKAGE_NAME
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/axum --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/axum
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml  --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-axum:${VERSION} ${REPOSITORY}/${BIN_NAME}-axum:latest

gotham:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/gotham --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/gotham
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-gotham:${VERSION} ${REPOSITORY}/${BIN_NAME}-gotham:latest

graphul:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/graphul --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/graphul
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-graphul:${VERSION} ${REPOSITORY}/${BIN_NAME}-graphul:latest

poem:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/poem --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/poem
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-poem:${VERSION} ${REPOSITORY}/${BIN_NAME}-poem:latest

rocket:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/rocket --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/rocket
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rocket:${VERSION} ${REPOSITORY}/${BIN_NAME}-rocket:latest

rouille:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/rouille --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/rouille
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-rouille:${VERSION} ${REPOSITORY}/${BIN_NAME}-rouille:latest

tonic-client:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/tonic-client --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/tonic-client
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/tonic-client"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-client:${VERSION} ${REPOSITORY}/${BIN_NAME}-tonic-client:latest

tonic-server:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/tonic-server --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/tonic-server
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/tonic-server"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-tonic-server:${VERSION} ${REPOSITORY}/${BIN_NAME}-tonic-server:latest

viz:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/viz --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/viz
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-viz:${VERSION} ${REPOSITORY}/${BIN_NAME}-viz:latest

warp:
    FROM +service
    ARG BIN_NAME
    ARG BUILD_DIR
    ARG REPOSITORY
    ARG VERSION
    COPY --platform=linux/amd64 (+build/warp --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /usr/local/bin/warp
    COPY --platform=linux/amd64 (+build/${BIN_NAME}.yaml --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --PACKAGE_NAME=${PACKAGE_NAME} --REPOSITORY=${REPOSITORY} --VERSION=${VERSION}) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}-warp:${VERSION} ${REPOSITORY}/${BIN_NAME}-warp:latest


aarch64-apple-darwin:
    ARG BIN_NAME
    ARG BUILD_FLAGS
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/
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
    ARG BIN_NAME
    ARG BUILD_FLAGS
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/
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
    ARG BIN_NAME
    ARG BUILD_FLAGS
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    CACHE target/
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
    ARG BIN_NAME
    ARG BUILD_FLAGS
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    CACHE target/
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
    ARG BIN_NAME
    ARG BUILD_FLAGS
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME}
    ENV RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc'
    CACHE target/
    RUN pwd
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
    ARG BIN_NAME
    ARG BUILD_FLAGS
    ARG DIST_DIR
    ARG PACKAGE_NAME
    ARG VERSION
    FROM kickable/builder:latest@sha256:2c8149ee36a109fc6449f0af6b7588a16a3dcd9067665d1052dfb492dec57d6b

    WORKDIR /usr/src/archive/aarch64-apple-darwin
    COPY (+aarch64-apple-darwin/* --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --BUILD_FLAGS=${BUILD_FLAGS} --PACKAGE_NAME=${PACKAGE_NAME} --VERSION=${VERSION}) .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 aarch64-apple-darwin.zip *
    RUN sha256sum aarch64-apple-darwin.zip > aarch64-apple-darwin.zip.sha256
    SAVE ARTIFACT aarch64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip
    SAVE ARTIFACT aarch64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/x86_64-apple-darwin
    COPY (+x86-64-apple-darwin/* --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --BUILD_FLAGS=${BUILD_FLAGS} --PACKAGE_NAME=${PACKAGE_NAME} --VERSION=${VERSION}) .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 x86_64-apple-darwin.zip *
    RUN sha256sum x86_64-apple-darwin.zip > x86_64-apple-darwin.zip.sha256
    SAVE ARTIFACT x86_64-apple-darwin.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip
    SAVE ARTIFACT x86_64-apple-darwin.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip.sha256

    WORKDIR /usr/src/archive/aarch64-unknown-linux-musl
    COPY (+aarch64-unknown-linux-musl/* --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --BUILD_FLAGS=${BUILD_FLAGS} --PACKAGE_NAME=${PACKAGE_NAME} --VERSION=${VERSION}) .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN tar -czvf aarch64-unknown-linux-musl.tar.gz *
    RUN sha256sum aarch64-unknown-linux-musl.tar.gz > aarch64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-unknown-linux-musl
    COPY (+x86-64-unknown-linux-musl/* --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --BUILD_FLAGS=${BUILD_FLAGS} --PACKAGE_NAME=${PACKAGE_NAME} --VERSION=${VERSION}) .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN tar -czvf x86_64-unknown-linux-musl.tar.gz *
    RUN sha256sum x86_64-unknown-linux-musl.tar.gz > x86_64-unknown-linux-musl.tar.gz.sha256
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz.sha256

    WORKDIR /usr/src/archive/x86_64-pc-windows-gnu
    COPY (+x86-64-pc-windows-gnu/* --BIN_NAME=${BIN_NAME} --BUILD_DIR=${BUILD_DIR} --BUILD_FLAGS=${BUILD_FLAGS} --PACKAGE_NAME=${PACKAGE_NAME} --VERSION=${VERSION}) .
    COPY README.md LICENSE.md CHANGELOG.md ${BIN_NAME}.yaml .
    RUN zip -9 x86_64-pc-windows-gnu.zip *
    RUN sha256sum x86_64-pc-windows-gnu.zip > x86_64-pc-windows-gnu.zip.sha256
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip.sha256 AS LOCAL ./${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip.sha256

