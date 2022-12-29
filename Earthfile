VERSION 0.6

FROM clux/muslrust:stable

shipyard:
    RUN cargo install cargo-chef --locked
    WORKDIR /kickable

prepare:
    FROM +shipyard
    COPY . .
    RUN cargo-chef chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json

cook:
    FROM +shipyard
    COPY +prepare/recipe.json recipe.json
    ENV CARGO=x86_64-unknown-linux-musl
    RUN cargo-chef chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

build:
    FROM +cook
    COPY . .
    RUN cargo build --verbose --release --target x86_64-unknown-linux-musl --all-features --examples --tests --bin kickable
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/kickable kickable

docker:
    FROM scratch
    RUN addgroup -S services && adduser -S kickable -G services
    COPY +build/kickable /usr/local/bin/kickable
    USER kickable
    CMD ["/usr/local/bin/kickable"]
    SAVE IMAGE --push defstream/kickable:latest