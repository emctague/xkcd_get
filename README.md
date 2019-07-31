# xkcd_get

`xkcd_get` provides a simple way of retrieving xkcd comic information.

## Cargo

```toml
[dependencies]
xkcd_get = "0.1.0"
```

## Usage

```rust
extern crate xkcd_get;
use xkcd_get::Comic;

fn main () {
    let data = Comic::get(10).unwrap();
    let data_latest = Comic::latest().unwrap();
    println!("Comic 10 was titled {}! xkcd is now on comic {}! Wow!", data.title, data_latest.num);
}
```