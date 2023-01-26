FROM docker:dind
RUN wget https://github.com/earthly/earthly/releases/download/v0.6.30/earthly-linux-amd64 -O /usr/local/bin/earthly && chmod +x /usr/local/bin/earthly && /usr/local/bin/earthly bootstrap
WORKDIR /usr/src/kickable
COPY src src
COPY proto proto
COPY examples examples
COPY scripts scripts
COPY Cargo.lock Cargo.toml Makefile build.rs ./

RUN

#ENTRYPOINT ["/bin/sh"]