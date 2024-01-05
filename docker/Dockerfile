ARG version=0.0.0

# setup build image + dependencies
FROM kickable/builder:latest@sha256:aef41d4f7b9b1797813b8c90593dd5d5a664be94e313e97f07a7b94310cdd8e7 AS build
ARG version
WORKDIR /usr/src/kickable
COPY src src
COPY proto proto
COPY examples examples
COPY scripts scripts
COPY i18n i18n
COPY kickable.yaml Cargo.lock Cargo.toml Makefile build.rs ./

# build kickable
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo build --release --bin kickable --all-features --locked --target x86_64-unknown-linux-musl

RUN addgroup --system kicker
RUN adduser --system --group kicker

# create a scratch image with the static binary
FROM scratch AS kickable
ARG version
COPY --from=build /etc/passwd /etc/passwd
USER kicker

COPY --from=build /usr/src/kickable/target/x86_64-unknown-linux-musl/release/kickable /usr/local/bin/kickable
COPY --from=build /usr/src/kickable/kickable.yaml /etc/kickable/config

USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]

LABEL description="This is the kickable container app that asks the question... Can you kick it?"
LABEL maintainer="Hector Gray <hector@hectorgray.com>"
LABEL version=$version
