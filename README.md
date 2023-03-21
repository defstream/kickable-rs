
<p align="center">
    <img src="https://media2.giphy.com/media/p3R62d6L0WYw0/200w.gif">
</p>

[![crate status](https://img.shields.io/crates/v/kickable.svg)](https://crates.io/crates/kickable)
[![branch status](https://github.com/defstream/kickable-rs/actions/workflows/main.yml/badge.svg?branch=main)
[![ci status](https://dl.circleci.com/status-badge/img/gh/defstream/kickable-rs/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/defstream/kickable-rs/tree/main)
[![travis status](https://app.travis-ci.com/defstream/kickable-rs.svg?branch=main)](https://app.travis-ci.com/defstream/kickable-rs)
[![coveralls status](https://coveralls.io/repos/github/defstream/kickable-rs/badge.svg?branch=main)](https://coveralls.io/github/defstream/kickable-rs?branch=main)
[![codecov status](https://codecov.io/gh/defstream/kickable-rs/branch/main/graph/badge.svg?token=JHAZGUBEC8)](https://codecov.io/gh/defstream/kickable-rs)
[![docs status](https://readthedocs.org/projects/kickable-rs/badge/?version=latest)](https://readthedocs.org/projects/kickable-rs)
[![OpenSSF Best Practices](https://bestpractices.coreinfrastructure.org/projects/7146/badge)](https://bestpractices.coreinfrastructure.org/projects/7146)

# kickable
**kickable** is a crate created to answer the age old question... "_Can I Kick It?_"
_This package is for showcase purposes only._

**What is a kickable?**
Currently only the word "it" is kickable.

## Library

### Install

Use the kickable library in your project by adding it to your Cargo.toml file.

```shell
$ cargo add kickable
```

### Usage

```rust
use kickable;

fn main() {
    let kickable = kickable::validate("it");
    println!("Can you kick it? {kickable}");
}
```

## CLI

### Install

Download the binary from the releases page and place it in your path, or if you have cargo installed.

```bash
$ cargo install kickable
```

### Usage
Obtain help about the kickable cli.

```shell
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

An example of the most basic usage of the kickable cli.
```shell
$ kickable "it"
Yes, yes you can.

```

### Configuration
Below is an example of an optional kickable configuration file. 

```yaml
# items that are kickable
items:
  - it   # English
  - él   # Spanish
  - el   # Spanish
  - それ  # Japanese
  - il   # French

# language preferred for i18n messages
lang: en-US # default language

# logging configuration
logging:
  level: 1 # 1: debug, 2: info, 3: warning, 4: error, 5: critical
  file: /var/log/kickable # log file path

# server configuration
server:
  addr: 0.0.0.0 # address to bind to
  port: 8080 # port number

# client configuration
client:
  addr: 127.0.0.1 # address to connect to
  port: 8080 # port number
```


## Maintainers
Hector Gray (<a href="https://hectorgray.com">@defstream</a>)

## Contribute
Pull Requests welcome. Please make sure all tests pass 😀

## License
Kickable by <a href="https://twitter.com/defstream">Hector Gray</a> is marked with CC0 1.0 Universal. To view a copy of this license, visit http://creativecommons.org/publicdomain/zero/1.0

[![license](https://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0kickable)

