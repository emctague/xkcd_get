# xkcd_get

[![Crates.io](https://img.shields.io/crates/v/xkcd_get?style=flat-square)](https://crates.io/crates/xkcd_get)
[![docs.rs](https://docs.rs/xkcd_get/badge.svg)](https://docs.rs/xkcd_get)

`xkcd_get` provides a simple way of retrieving xkcd comic information.

## Cargo

```toml
[dependencies]
xkcd_get = "0.1.2"
```

## Usage

```rust
use xkcd_get::Comic;

fn main () {
    let data = Comic::get(10).unwrap();
    let data_latest = Comic::latest().unwrap();
    println!("Comic 10 was titled {}! xkcd is now on comic {}! Wow!", data.title, data_latest.num);
}
```