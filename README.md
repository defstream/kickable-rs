
<p align="center">
<img src="https://media2.giphy.com/media/p3R62d6L0WYw0/200w.gif">
</p>

# kickable
**kickables** is a package created to answer the age old question... "_Can I Kick It?_"
_This package is for showcase purposes only._

**What is a kickable?**
Currently only the word "it" is kickable.

# Library

### Install

Download the binary from the releases page and place it in your path, or if you have cargo installed. 
```shell
$ cargo install kickable@latest
```````

### Usage

```rust
use kickable;

fn main() {
    let kickable = kickable::validate("it");
    println!("Can I kick it? {kickable}");
}
```

# CLI

### Install

```bash
$ cargo install "kickable@latest"
```

### Usage

```shell
$ kickable "it"
```

# Maintainers
Hector Gray (Twitter: <a href="https://twitter.com/defstream">@defstream</a>)

# Contribute
Pull Requests welcome. Please make sure all tests pass 😀

# License
MIT