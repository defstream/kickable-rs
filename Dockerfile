FROM rust:1.66 as build
WORKDIR /usr/src/kickable
COPY src src
COPY examples examples
COPY Cargo.lock Cargo.toml Makefile ./
RUN make build

FROM debian:buster-slim as kickable
COPY --from=build /usr/src/kickable/target/release/kickable /usr/local/bin/kickable
USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]
