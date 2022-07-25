tiny-gradient
=============

[<img alt="github" src="https://img.shields.io/badge/github-zhiburt/tiny--gradient-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zhiburt/tiny-gradient/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tiny-gradient.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tiny-gradient)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tiny--gradient-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tiny-gradient)

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/20165848/180790191-63252827-64ef-4d09-8a01-384f78bf8bb1.png">
  <img alt="Preview" src="https://user-images.githubusercontent.com/20165848/180790008-b1100184-ce27-42c9-8c60-8628e5833794.png">
</picture>

## Usage

```rust
use tiny_gradient::{Gradient, GradientStr, RGB};

let text = "Hello World!";

// Use custom gradient
let colored = text.gradient([RGB::new(0x01, 0x00, 0x00), RGB::new(0xDA, 0x00, 0xFF)]);
println!("{}", colored);

// Use built-in gradient.
let colored = text.gradient(Gradient::Forest);
println!("{}", colored);
```

## Notes

All the credit should go to https://stackoverflow.com/questions/22607043/color-gradient-algorithm/.

Some insiration were taken from https://github.com/bokub/gradient-string.

There's an analogues library https://crates.io/crates/colorgrad.
I wasn't aware at the begining that it exists.
It seems to be more mature so you may need to take a look at it.
