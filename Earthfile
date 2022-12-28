VERSION 0.6

FROM rust:latest

shipyard:
    RUN cargo install cargo-binstall
    RUN cargo binstall cross -y --log-level debug
    WORKDIR /kickable

build:
    FROM +shipyard
    COPY . .
    RUN cargo build --verbose --release --all-features --examples --tests --bin kickable
    SAVE ARTIFACT target/release/kickable kickable

docker:
    COPY +build/kickable .
    ENTRYPOINT ["/kickable/kickable"]
    SAVE IMAGE --push defstream/kickable:latest

