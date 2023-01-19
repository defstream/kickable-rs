VERSION 0.6

ARG VERSION = 0.0.0
ARG ORG = defstream
ARG BIN_NAME = kickable
ARG PACKAGE_NAME = kickable-rs

ARG BUILD_DIR = target/x86_64-unknown-linux-musl/release
ARG BUILD_FLAGS = --release --all-features --locked

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
    FROM scratch
    COPY +build/${BIN_NAME} /usr/local/bin/kickable
    CMD ["/usr/local/bin/kickable"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}:${VERSION}

aarch64-apple-darwin:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target aarch64-apple-darwin
    SAVE ARTIFACT target/aarch64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}

aarch64-unknown-linux-musl:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target aarch64-unknown-linux-musl
    SAVE ARTIFACT target/aarch64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}

x86-64-apple-darwin:
    FROM +source
    RUN cargo build ${BUILD_FLAGS} --target x86_64-apple-darwin
    SAVE ARTIFACT target/x86_64-apple-darwin/release/${BIN_NAME} ${BIN_NAME}

x86-64-unknown-linux-musl:
    FROM +source
    ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
    RUN cargo build ${BUILD_FLAGS} --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/${BIN_NAME} ${BIN_NAME}

x86-64-pc-windows-gnu:
    FROM +source
    ENV RUSTFLAGS='-C linker=x86_64-w64-mingw32-gcc'
    RUN cargo build ${BUILD_FLAGS}  --target x86_64-pc-windows-gnu
    SAVE ARTIFACT target/x86_64-pc-windows-gnu/release/${BIN_NAME}.exe ${BIN_NAME}.exe

service:
    ARG port=31337
    FROM scratch
    EXPOSE $port

axum:
    FROM +service
    COPY +build/axum /usr/local/bin/axum
    ENTRYPOINT ["/usr/local/bin/axum"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-axum:${VERSION}

gotham:
    FROM +service
    COPY +build/gotham /usr/local/bin/gotham
    ENTRYPOINT ["/usr/local/bin/gotham"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-gotham:${VERSION}

graphul:
    FROM +service
    COPY +build/graphul /usr/local/bin/graphul
    ENTRYPOINT ["/usr/local/bin/graphul"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-graphul:${VERSION}

poem:
    FROM +service
    COPY +build/poem /usr/local/bin/poem
    ENTRYPOINT ["/usr/local/bin/poem"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-poem:${VERSION}

rocket:
    FROM +service
    COPY +build/rocket /usr/local/bin/rocket
    ENTRYPOINT ["/usr/local/bin/rocket"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-rocket:${VERSION}

rouille:
    FROM +service
    COPY +build/rouille /usr/local/bin/rouille
    ENTRYPOINT ["/usr/local/bin/rouille"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-rouille:${VERSION}

salvo:
    FROM +service
    COPY +build/salvo /usr/local/bin/salvo
    ENTRYPOINT ["/usr/local/bin/salvo"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-salvo:${VERSION}

tonic-client:
    FROM +service
    COPY +build/tonic-client /usr/local/bin/tonic-client
    ENTRYPOINT ["/usr/local/bin/tonic-client"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-tonic-client:${VERSION}

tonic-server:
    FROM +service
    COPY +build/tonic-server /usr/local/bin/tonic-server
    ENTRYPOINT ["/usr/local/bin/tonic-server"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-tonic-server:${VERSION}

trillium:
    FROM +service
    COPY +build/trillium /usr/local/bin/trillium
    ENTRYPOINT ["/usr/local/bin/trillium"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-trillium:${VERSION}

viz:
    FROM +service
    COPY +build/viz /usr/local/bin/viz
    ENTRYPOINT ["/usr/local/bin/viz"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-viz:${VERSION}

warp:
    FROM +service
    COPY +build/warp /usr/local/bin/warp
    ENTRYPOINT ["/usr/local/bin/warp"]
    SAVE IMAGE --push ${ORG}/${BIN_NAME}-warp:${VERSION}

archive:
    FROM +builder

    WORKDIR /usr/src/archive
    COPY README.md LICENSE CHANGELOG.md .

    WORKDIR /usr/src/archive/aarch64-apple-darwin
    COPY +aarch64-apple-darwin/* .
    RUN zip aarch-apple-darwin.zip * ../README.md ../CHANGELOG.md ../LICENSE
    SAVE ARTIFACT aarch-apple-darwin.zip ${PACKAGE_NAME}_${VERSION}_aarch-apple-darwin.zip

    WORKDIR /usr/src/archive/x86_64-apple-darwin
    COPY +x86-64-apple-darwin/* .
    RUN zip x86_64-apple-darwin.zip * ../README.md ../CHANGELOG.md ../LICENSE
    SAVE ARTIFACT x86_64-apple-darwin.zip ${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip

    WORKDIR /usr/src/archive/aarch64-unknown-linux-musl
    COPY +aarch64-unknown-linux-musl/* .
    RUN tar -czvf aarch64-unknown-linux-musl.tar.gz * ../README.md ../CHANGELOG.md ../LICENSE
    SAVE ARTIFACT aarch64-unknown-linux-musl.tar.gz ${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz

    WORKDIR /usr/src/archive/x86_64-unknown-linux-musl
    COPY +x86-64-unknown-linux-musl/* .
    RUN tar -czvf x86_64-unknown-linux-musl.tar.gz * ../README.md ../CHANGELOG.md ../LICENSE
    SAVE ARTIFACT x86_64-unknown-linux-musl.tar.gz ${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz

    WORKDIR /usr/src/archive/x86_64-pc-windows-gnu
    COPY +x86-64-pc-windows-gnu/* .
    RUN zip x86_64-pc-windows-gnu.zip * ../README.md ../CHANGELOG.md ../LICENSE
    SAVE ARTIFACT x86_64-pc-windows-gnu.zip ${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip

release:
    FROM debian:buster-slim
    WORKDIR /usr/src/${PACKAGE_NAME}-release
    COPY scripts/release-setup.sh .

    COPY +archive/${PACKAGE_NAME}_${VERSION}_aarch-apple-darwin.zip ./${PACKAGE_NAME}_${VERSION}_aarch-apple-darwin.zip
    COPY +archive/${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip ./${PACKAGE_NAME}_${VERSION}_x86_64-apple-darwin.zip
    COPY +archive/${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz ./${PACKAGE_NAME}_${VERSION}_aarch64-unknown-linux-musl.tar.gz
    COPY +archive/${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz ./${PACKAGE_NAME}_${VERSION}_x86_64-unknown-linux-musl.tar.gz
    COPY +archive/${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip ./${PACKAGE_NAME}_${VERSION}_x86_64-pc-windows-gnu.zip
    RUN ./release-setup.sh
    SAVE IMAGE $ORG/${PACKAGE_NAME}-release:${VERSION}
