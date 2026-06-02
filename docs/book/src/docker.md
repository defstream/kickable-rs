# Docker

The container images are built from static, cross-compiled binaries packaged
into a `scratch` image — there is no Rust toolchain in the final image, so it is
tiny. Compilation is done on the host with
[cross-rs](https://github.com/cross-rs/cross); the `Dockerfile` only copies the
resulting binary in.

## Build & run

```shell
make docker
```

This runs:

```shell
cross build --release --bin kickable --all-features --locked \
  --target x86_64-unknown-linux-musl
docker build --platform linux/amd64 -f docker/Dockerfile -t defstream/kickable:latest .
```

Then run it as a CLI:

```shell
$ docker run --rm --platform linux/amd64 defstream/kickable:latest it
Yes, yes you can.
```

> On Apple Silicon the cross-rs images are `linux/amd64`-only, so the Makefile
> targets set `DOCKER_DEFAULT_PLATFORM=linux/amd64` to run them under emulation.

## How the build works

- **`rust-toolchain.toml`** pins the Rust version (latest stable) for local, CI,
  and the cross-rs build containers.
- **`Cross.toml`** pins the cross-rs images to the `:main` tag (a current glibc
  compatible with the latest Rust).
- **`protoc`** is vendored via `protoc-bin-vendored`, so no `protobuf-compiler`
  is needed in any image.
- **Linux/Windows** targets build in cross-rs images; **macOS** targets build in
  an osxcross image (`docker/Dockerfile.builder`, used inline by the Earthfile)
  because cross-rs has no macOS image.

## Cross-compiled images & archives

Per-target packaging Dockerfiles live in `docker/Dockerfile.<target>`, and the
multi-framework service images plus distribution archives are produced via
[Earthly](https://earthly.dev):

```shell
make cross/build              # all target binaries via cross-rs
make earthly/build            # distribution archives -> ./dist
make earthly/docker/services  # one image per framework server
```

Distribution archives are written to `dist/` with reproducible (pinned)
timestamps, so re-running a build produces byte-identical artifacts.
