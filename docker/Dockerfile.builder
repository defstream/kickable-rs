ARG version=0.0.0
# setup build image + dependencies
FROM joseluisq/rust-linux-darwin-builder:1.68@sha256:52164d6e63991bca5a9ec4764e638bb2c7f846e6a21c866123aed6460aab32ab as shipyard
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

