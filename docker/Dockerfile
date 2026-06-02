ARG version=0.0.0

# The static musl binary is cross-compiled on the host with cross-rs (see the
# `docker` target in the Makefile), so this image only needs to package it.
# Build first with:
#   cross build --release --bin kickable --all-features --locked \
#     --target x86_64-unknown-linux-musl
FROM scratch AS kickable
ARG version

COPY target/x86_64-unknown-linux-musl/release/kickable /usr/local/bin/kickable
COPY kickable.yaml /etc/kickable/config

USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]

LABEL description="This is the kickable container app that asks the question... Can you kick it?"
LABEL maintainer="Hector Gray <hector@hectorgray.com>"
LABEL version=$version
