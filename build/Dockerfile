ARG DOCKER_IMAGE=rust:1.66

FROM $DOCKER_IMAGE as shipyard
RUN apt-get update -y
RUN apt-get install protobuf-compiler -y

FROM shipyard as build
WORKDIR /usr/src/kickable
COPY src src
COPY proto proto
COPY examples examples
COPY scripts scripts
COPY Cargo.lock Cargo.toml Makefile build.rs ./
RUN scripts/ubuntu-deps.sh
RUN make build

FROM scratch as kickable
ARG RUST_TARGET
ENV RUST_TARGET=x86_64-unknown-linux-musl
COPY --from=build /usr/src/kickable/target/$RUST_TARGET/release/kickable /usr/local/bin/kickable
USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]
