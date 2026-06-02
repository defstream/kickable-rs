# Usage

`kickable` answers one question — _Can I Kick It?_ — as a library, a CLI, and a
small HTTP/gRPC service implemented across nine Rust web frameworks.

## Library

```rust
use kickable;

fn main() {
    let kickable = kickable::validate("it");
    println!("Can you kick it? {kickable}");
}
```

The crate exposes two functions:

- `validate(input: &str) -> bool` — true when the input is kickable (`"it"`).
- `validate_amongst(input: &str, items: Vec<String>) -> bool` — true when the
  input matches any item in the supplied list (used for i18n / custom words).

## CLI

```shell
$ kickable it
Yes, yes you can.

$ kickable --help
kickable is a crate created to answer the age old question... Can I kick it?

Usage: kickable [OPTIONS] <ITEM>

Arguments:
  <ITEM>  The item to check for kick-ability

Options:
  -c, --config <CONFIG>  The path to the configuration file [default: kickable.yaml]
  -h, --help             Print help
  -V, --version          Print version
```

The CLI exits `0` when the item is kickable and non-zero otherwise, so it
composes in scripts:

```shell
$ kickable it && echo "kick confirmed"
```

## Configuration

An optional YAML config customizes the kickable items, language, logging, and
the server/client used by the framework binaries:

```yaml
# items that are kickable
items:
  - it   # English
  - él   # Spanish
  - el   # Spanish
  - それ  # Japanese
  - il   # French

# language preferred for i18n messages
lang: en-US

# logging configuration
logging:
  level: 1 # 1: debug, 2: info, 3: warning, 4: error, 5: critical
  file: /var/log/kickable

# server configuration (used by the framework binaries)
server:
  addr: 0.0.0.0
  port: 8080

# client configuration (used by the tonic client)
client:
  addr: 0.0.0.0
  port: 8080
```

## Servers

The same `GET /:item` service is implemented in each of the following
frameworks; run any of them with `cargo run --bin <name>`:

| Binary         | Framework                |
| -------------- | ------------------------ |
| `axum`         | axum (tokio · tower)     |
| `gotham`       | gotham                   |
| `graphul`      | graphul                  |
| `poem`         | poem                     |
| `rocket`       | rocket                   |
| `rouille`      | rouille (synchronous)    |
| `viz`          | viz (hyper 1.0)          |
| `warp`         | warp                     |
| `tonic-server` | tonic (gRPC) + `tonic-client` |

Each server reads the `server` block from `kickable.yaml`. For example:

```shell
$ cargo run --bin axum
$ curl localhost:8080/it
true
```
