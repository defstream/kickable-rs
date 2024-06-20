ARG version=0.0.0
# setup build image + dependencies
FROM joseluisq/rust-linux-darwin-builder:1.79@sha256:a005510afb706352c8243a6c2510fdce057665bfc37efeeb5dce421d4a39ea74
ARG version

COPY scripts/build-setup.sh .
RUN ./build-setup.sh
RUN curl https://github.com/earthly/earthly/releases/download/v0.7.23/earthly-linux-amd64 --output /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
WORKDIR /usr/src
RUN apt clean

# metadata
LABEL org.opencontainers.image.vendor="Hector Gray <hector@hectorgray.com>" \
    org.opencontainers.image.url="https://github.com/defstream/kickable-rs" \
    org.opencontainers.image.title="Rust Linux / Darwin / Windows Builder" \
    org.opencontainers.image.description="Cross Compile Rust Applications" \
    org.opencontainers.image.version="$version" \
    org.opencontainers.image.documentation="https://github.com/defstream/kickable-rs"
