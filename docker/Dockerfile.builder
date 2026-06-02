# macOS (Apple Darwin) cross-compilation builder.
#
# Linux and Windows targets are built with cross-rs, but cross-rs has no macOS
# image (no osxcross), so the *-apple-darwin targets are built from this image,
# whose joseluisq base bundles osxcross + the macOS SDK. The base pins Rust
# 1.89, which is below the MSRV of current deps, so we install the latest stable
# toolchain on top. protoc is vendored in build.rs, so no protobuf-compiler is
# needed here.
ARG version=0.0.0
# setup build image + dependencies
FROM joseluisq/rust-linux-darwin-builder:1.89.0@sha256:a3e69706ba273c08a23ab76be5c2d98a9075c33c2504e4fb17d36a64ae713ff2
ARG version

# Upgrade to the latest stable Rust (osxcross in the base is toolchain-agnostic)
# and add the macOS targets' std for the new toolchain.
RUN rustup toolchain install stable --profile minimal \
    && rustup default stable \
    && rustup target add aarch64-apple-darwin x86_64-apple-darwin
WORKDIR /usr/src
RUN apt-get clean

# metadata
LABEL org.opencontainers.image.vendor="Hector Gray <hector@hectorgray.com>" \
    org.opencontainers.image.url="https://github.com/defstream/kickable-rs" \
    org.opencontainers.image.title="Rust Apple Darwin (osxcross) Builder" \
    org.opencontainers.image.description="Cross compile kickable for macOS targets" \
    org.opencontainers.image.version="$version" \
    org.opencontainers.image.documentation="https://github.com/defstream/kickable-rs"
