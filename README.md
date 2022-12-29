
<p align="center">
<img src="https://media2.giphy.com/media/p3R62d6L0WYw0/200w.gif">
</p>

[![crate status](https://img.shields.io/crates/v/kickable.svg)](https://crates.io/crates/kickable)
![branch status](https://github.com/defstream/kickable-rs/actions/workflows/push_pull.yml/badge.svg?branch=main)
[![ci status](https://dl.circleci.com/status-badge/img/gh/defstream/kickable-rs/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/defstream/kickable-rs/tree/main)
[![coverage status](https://coveralls.io/repos/github/defstream/kickable-rs/badge.svg?branch=main)](https://coveralls.io/github/defstream/kickable-rs?branch=main)



# kickable

**kickable** is a crate created to answer the age old question... "_Can I Kick It?_"
_This package is for showcase purposes only._

**What is a kickable?**
Currently only the word "it" is kickable.

# Library

### Install

Download the binary from the releases page and place it in your path, or if you have cargo installed. 
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

# CLI

### Install

```bash
$ cargo install kickable
```

### Usage

```shell
$ kickable "it"
Yes, yes you can.

```

# Maintainers
Hector Gray (Twitter: <a href="https://twitter.com/defstream">@defstream</a>)

# Contribute
Pull Requests welcome. Please make sure all tests pass 😀

# License
MIT