tiny-gradient
=============

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