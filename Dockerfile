FROM rust:1.66 as shipyard
RUN apt-get update -y
RUN apt-get install protobuf-compiler -y

FROM shipyard as build
WORKDIR /usr/src/kickable
COPY src src
COPY proto proto
COPY examples examples
COPY Cargo.lock Cargo.toml Makefile build.rs ./
RUN make build

FROM debian:buster-slim as kickable
COPY --from=build /usr/src/kickable/target/release/kickable /usr/local/bin/kickable
USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]
