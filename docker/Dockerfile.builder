ARG version=0.0.0
# setup build image + dependencies
FROM joseluisq/rust-linux-darwin-builder:1.72@sha256:8ec817ea03b405f062c224ac6de5f6738bf89229335606694fa252258264c440 as shipyard
COPY scripts/build-setup.sh .
RUN ./build-setup.sh
RUN curl https://github.com/earthly/earthly/releases/download/v0.7.2/earthly-linux-amd64 --output /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap

WORKDIR /user/src

# metadata
LABEL org.opencontainers.image.vendor="Hector Gray <hector@hectorgray.com>" \
    org.opencontainers.image.url="https://github.com/defstream/rust-builder" \
    org.opencontainers.image.title="Rust Linux / Darwin / Windows Builder" \
    org.opencontainers.image.description="Cross compile linux applications" \
    org.opencontainers.image.version="$version" \
    org.opencontainers.image.documentation="https://github.com/defstream/rust-builder"

