# tiengviet

Convert vietnamese sign to unsign easily like eat 🥞🥞🥞.

[![Quick check](https://github.com/tvc12/tiengviet-rust/actions/workflows/quick_check.yml/badge.svg)](https://github.com/tvc12/tiengviet-rust/actions/workflows/quick_check.yml)
![Contribute](https://img.shields.io/github/contributors/tvc12/tiengviet-rust.svg?style=flat-square)
![GitHub](https://img.shields.io/github/license/tvc12/tiengviet-rust.svg?style=flat-square)
[![Hits-of-Code](https://hitsofcode.com/github/tvc12/tiengviet-rust?branch=main)](https://hitsofcode.com/github/tvc12/tiengviet-rust?branch=main)

🇻🇳 Công cụ chuyển đổi Tiếng Việt có dấu sang không dấu dễ hơn là ăn bánh 🥞🥞🥞🥞.

### Getting Started

#### Install

```sh
cargo add tiengviet
```

#### Usage

```rust
use tiengviet::TiengViet;

let tiengviet = TiengViet::new();

let result: String = tiengviet::parse(&String.from("Xin chào việt nam");
// Xin chao viet nam
```

### You might also like

- [vietnamese](https://github.com/harrytran103/vietnamese) - ✂️🇻🇳 A helpful tool for removing Vietnamese accents in nodejs.
- [tiengviet](https://github.com/tvc12/tiengviet) - ✂️🇻🇳 An implement tiengviet in dart.


### Contributors

| [![Vi Chi Thien](https://github.com/tvc12.png?size=100)](https://github.com/tvc12) |
| :--------------------------------------------------------------------------------: |
|                      [Vi Chi Thien](https://github.com/tvc12)                      |


### License

[MIT @tvc12](./LICENSE)
