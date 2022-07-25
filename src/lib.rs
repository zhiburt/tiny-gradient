//! `tiny-gradient` colorful gradients for your terminal.
//!
//! ### Gradient usage
//!
//! ```
//! use tiny_gradient::{Gradient, GradientStr};
//!
//! let text = "Hello World!";
//!
//! let colored = text.gradient(Gradient::Forest);
//!
//! println!("{}", colored);
//! ```
//!
//! ### RGB usage
//!
//! ```
//! use tiny_gradient::{RGB, GradientStr};
//!
//! let text = "Hello World!";
//!
//! let colored = text.gradient([RGB::new(0x01, 0x00, 0x00), RGB::new(0xDA, 0x00, 0xFF)]);
//!
//! println!("{}", colored);
//! ```
//!
//! ### Generator usage
//!
//! ```
//! use tiny_gradient::{gradient::Gradient, RGB};
//!
//! let mut gradient = Gradient::new(RGB::new(0x01, 0x00, 0x00), RGB::new(0xDA, 0x00, 0xFF), 10);
//!
//! for color in gradient {
//!     println!("{:?}", color);
//! }
//! ```

#![no_std]
#![warn(
    missing_docs,
    rustdoc::missing_doc_code_examples,
    rust_2018_idioms,
    rust_2021_prelude_collisions
)]

mod display;
mod gradients;
mod rgb;

pub mod gradient;

pub use crate::display::{GradientDisplay, GradientStr};
pub use gradients::Gradient;
pub use rgb::RGB;
