# tiengviet

Convert vietnamese sign to unsign easily like eat 🥞🥞🥞.

![Build Status](https://img.shields.io/travis/com/tvc12/tiengviet-rust?style=flat-square)
![Contribute](https://img.shields.io/github/contributors/tvc12/tiengviet-rust.svg?style=flat-square)
![GitHub](https://img.shields.io/github/license/tvc12/tiengviet-rust.svg?style=flat-square)
[![Hits-of-Code](https://hitsofcode.com/github/tvc12/tiengviet-rust)](https://hitsofcode.com/view/github/tvc12/tiengviet-rust?style=flat-square)

🇻🇳 Công cụ chuyển đổi Tiếng Việt có dấu sang không dấu dễ hơn là ăn bánh 🥞🥞🥞🥞.

### Getting Started

#### Install

```sh
cargo add tiengviet
```

#### Usage

```rust
use tiengviet::TiengViet;

println!("result: {}", TiengViet::parse("Xin chào việt nam")); // result: Xin chao viet nam
```

### You might also like

- [vietnamese](https://github.com/harrytran103/vietnamese) - ✂️🇻🇳 A helpful tool for removing Vietnamese accents in nodejs.
- [vietnamese](https://github.com/tvc12/tiengviet) - ✂️🇻🇳 An implement tiengviet in dart.


### Contributors

| [![Vi Chi Thien](https://github.com/tvc12.png?size=100)](https://github.com/tvc12) |
| :--------------------------------------------------------------------------------: |
|                      [Vi Chi Thien](https://github.com/tvc12)                      |


### License

[BSD 3-Clause @tvc12](./LICENSE)
