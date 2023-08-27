ARG version=0.0.0

# setup build image + dependencies
FROM kickable/builder:latest@sha256:d11ea65dfd2bea1089ee6b60e85ba8254cd7ba8b32aa0ac12fdd31f00432efe2 as build
WORKDIR /usr/src/kickable
COPY src src
COPY proto proto
COPY examples examples
COPY scripts scripts
COPY i18n i18n
COPY Cargo.lock Cargo.toml Makefile build.rs ./

# build kickable
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo build --release --bin kickable --all-features --locked --target x86_64-unknown-linux-musl

# create a scratch image with the static binary
FROM scratch as kickable
COPY --from=build /usr/src/kickable/target/x86_64-unknown-linux-musl/release/kickable /usr/local/bin/kickable
USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]

LABEL description="This is the kickable container app that asks the question... Can you kick it?"
LABEL maintainer="Hector Gray <hector@hectorgray.com>"
LABEL version=$version
