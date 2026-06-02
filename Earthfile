VERSION --global-cache 0.8

ARG --global BUILD_DIR=target/x86_64-unknown-linux-musl/release
ARG --global BUILD_FLAGS=--verbose --release --all-features --locked
ARG --global BIN_NAME=kickable
ARG --global DIST_DIR=dist
ARG --global REPOSITORY=defstream
ARG --global PACKAGE_NAME=kickable-rs
ARG --global VERSION=0.0.0
ARG --global LABEL_MAINTAINER=Hector Gray <hector@hectorgray.com>
ARG --global port=31337

# Build images. Linux + Windows use cross-rs toolchain images (the `:main` tag
# carries a recent glibc compatible with the latest Rust). macOS has no cross-rs
# image, so the Apple targets use the osxcross builder (docker/Dockerfile.builder
# -> kickable/builder, latest stable Rust + osxcross). protoc is vendored in
# build.rs, and each image configures its own target linker, so no RUSTFLAGS
# linker overrides are needed.
ARG --global IMAGE_X86_64_LINUX_MUSL=ghcr.io/cross-rs/x86_64-unknown-linux-musl:main
ARG --global IMAGE_AARCH64_LINUX_MUSL=ghcr.io/cross-rs/aarch64-unknown-linux-musl:main
ARG --global IMAGE_X86_64_WINDOWS_GNU=ghcr.io/cross-rs/x86_64-pc-windows-gnu:main

benchmark:
    FROM debian:buster-slim
    COPY scripts/benchmark-setup.sh scripts/benchmark.sh .
    RUN ./benchmark-setup.sh
    ENTRYPOINT ["benchmark.sh"]

source:
    ARG FROM_IMAGE=${IMAGE_X86_64_LINUX_MUSL}
    # cross-rs images are published for linux/amd64 only; pin the build platform
    # so they resolve on arm64 hosts/satellites (the Rust target is selected via
    # --target, so the container arch is independent of the compile target).
    FROM --platform=linux/amd64 ${FROM_IMAGE}

    # cross-rs images ship only the C cross-toolchain, not Rust (the `cross` CLI
    # normally mounts the host toolchain). Install rustup when the base lacks it
    # (the osxcross builder already has Rust). rust-toolchain.toml then pins the
    # channel + target std on first cargo invocation.
    ENV PATH="/root/.cargo/bin:${PATH}"
    RUN command -v cargo >/dev/null 2>&1 || ( \
            (command -v curl >/dev/null 2>&1 || (apt-get update && apt-get install --assume-yes --no-install-recommends curl ca-certificates)) && \
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none --profile minimal )

    WORKDIR /usr/src/${PACKAGE_NAME}
    COPY --keep-ts --dir i18n scripts examples proto src .
    COPY --keep-ts kickable.yaml Cargo.lock Cargo.toml Makefile build.rs rust-toolchain.toml README.md CHANGELOG.md LICENSE.md .

# The Apple Darwin targets need osxcross, which cross-rs does not provide. Build
# the osxcross builder (docker/Dockerfile.builder: joseluisq + latest stable
# Rust + osxcross) inline via FROM DOCKERFILE, so no pre-published kickable/builder
# image is required and it can never drift stale.
darwin-source:
    FROM DOCKERFILE --platform=linux/amd64 -f docker/Dockerfile.builder .
    WORKDIR /usr/src/${PACKAGE_NAME}
    COPY --keep-ts --dir i18n scripts examples proto src .
    COPY --keep-ts kickable.yaml Cargo.lock Cargo.toml Makefile build.rs rust-toolchain.toml README.md CHANGELOG.md LICENSE.md .

build:
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME} --FROM_IMAGE=${IMAGE_X86_64_LINUX_MUSL}
    RUN cargo build ${BUILD_FLAGS} --target x86_64-unknown-linux-musl
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
    COPY (+build/kickable) /usr/local/bin/${BIN_NAME}
    COPY (+build/kickable.yaml) /etc/${BIN_NAME}/config
    ENTRYPOINT ["/usr/local/bin/kickable"]
    SAVE IMAGE --push ${REPOSITORY}/${BIN_NAME}:${VERSION} ${REPOSITORY}/${BIN_NAME}:latest

kickable:
    BUILD +kickable-build

service:
    FROM scratch
    EXPOSE $port

services:
    BUILD +axum
    BUILD +gotham
    BUILD +graphul
    BUILD +poem
    BUILD +rocket
    BUILD +rouille
    BUILD +tonic-client
    BUILD +tonic-server
    BUILD +viz
    BUILD +warp

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
    FROM +darwin-source --PACKAGE_NAME=${PACKAGE_NAME}
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
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME} --FROM_IMAGE=${IMAGE_AARCH64_LINUX_MUSL}
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
    FROM +darwin-source --PACKAGE_NAME=${PACKAGE_NAME}
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
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME} --FROM_IMAGE=${IMAGE_X86_64_LINUX_MUSL}
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
    FROM +source --PACKAGE_NAME=${PACKAGE_NAME} --FROM_IMAGE=${IMAGE_X86_64_WINDOWS_GNU}
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

# Distribution archives. Binaries come from the per-target build targets above;
# this target only packages them, so it runs on a small Debian image with zip.
archive:
    FROM debian:stable-slim
    RUN apt-get update && apt-get install --assume-yes zip && rm -rf /var/lib/apt/lists/*

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
