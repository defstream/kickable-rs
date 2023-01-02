FROM clux/muslrust:stable as shipyard

FROM shipyard as drydock
WORKDIR /usr/src/kickable
COPY src src
COPY examples examples
COPY Cargo.lock Cargo.toml Makefile ./

FROM drydock as builder
RUN make build

FROM scratch
COPY --from=builder /usr/src/kickable/target/x86_64-unknown-linux-musl/release/kickable /usr/local/bin/kickable
USER 1000
ENTRYPOINT ["/usr/local/bin/kickable"]
