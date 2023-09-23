ARG version=0.0.0
# setup build image + dependencies
FROM joseluisq/rust-linux-darwin-builder:1.72.1@sha256:aea533fbef676e640c4661594a7d146f7062478705486b6e31c1f85547d2f83b as shipyard
COPY scripts/build-setup.sh .
RUN ./build-setup.sh
RUN curl https://github.com/earthly/earthly/releases/download/v0.7.19/earthly-linux-amd64 --output /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap

WORKDIR /usr/src

# metadata
LABEL org.opencontainers.image.vendor="Hector Gray <hector@hectorgray.com>" \
    org.opencontainers.image.url="https://github.com/defstream/rust-builder" \
    org.opencontainers.image.title="Rust Linux / Darwin / Windows Builder" \
    org.opencontainers.image.description="Cross Compile Rust Applications" \
    org.opencontainers.image.version="$version" \
    org.opencontainers.image.documentation="https://github.com/defstream/rust-builder"
