//! [![github](https://img.shields.io/static/v1?logo=github&label=github&message=mazznoer/csscolorparser-rs&color=8da0cb)](https://github.com/mazznoer/csscolorparser-rs/)
//! [![crates.io](https://img.shields.io/crates/v/csscolorparser.svg)](https://crates.io/crates/csscolorparser)
//!
//! # Overview
//!
//! Rust library to parse CSS color string as defined in the W3C's [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).
//!
//! ## Supported Color Format
//!
//! * [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
//! * RGB hexadecimal
//!      + Short format `#rgb`
//!      + Short format with alpha `#rgba`
//!      + Long format `#rrggbb`
//!      + Long format with alpha `#rrggbbaa`
//! * `rgb()` and `rgba()`
//! * `hsl()` and `hsla()`
//! * `hwb()`
//! * `hsv()` - not in CSS standard.
//!
//! Not yet supported: `lab()`, `lch()`.
//!
//! ### Example Color Format
//!
//! ```text
//! transparent
//! gold
//! rebeccapurple
//! lime
//! #0f0
//! #0f0f
//! #00ff00
//! #00ff00ff
//! rgb(0,255,0)
//! rgb(0% 100% 0%)
//! rgb(0 255 0 / 100%)
//! rgba(0,255,0,1)
//! hsl(120,100%,50%)
//! hsl(120deg 100% 50%)
//! hsl(-240 100% 50%)
//! hsl(-240deg 100% 50%)
//! hsl(0.3333turn 100% 50%)
//! hsl(133.333grad 100% 50%)
//! hsl(2.0944rad 100% 50%)
//! hsla(120,100%,50%,100%)
//! hwb(120 0% 0%)
//! hwb(480deg 0% 0% / 100%)
//! hsv(120,100%,100%)
//! hsv(120deg 100% 100% / 100%)
//! ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//!
//! ```toml
//! csscolorparser = "0.4.0"
//! ```
//!
//! ## Examples
//!
//! Using [`csscolorparser::parse()`](fn.parse.html) function.
//!
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let c = csscolorparser::parse("rgb(100%,0%,0%)")?;
//!
//! assert_eq!(c.rgba(), (1., 0., 0., 1.));
//! assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(c.to_hex_string(), "#ff0000");
//! assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
//! # Ok(())
//! # }
//! ```
//!
//! Using `parse()` method on string.
//!
//! ```rust
//! use csscolorparser::Color;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! let c = "#ff00007f".parse::<Color>()?;
//!
//! assert_eq!(c.rgba_u8(), (255, 0, 0, 127));
//! assert_eq!(c.to_hex_string(), "#ff00007f");
//! # Ok(())
//! # }
//! ```
//!
//! Using [`Color::from_html()`](struct.Color.html#method.from_html).
//!
//! ```rust
//! use csscolorparser::Color;
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//!
//! let c = Color::from_html("rgb(135,206,235)")?;
//!
//! assert_eq!(c.rgba_u8(), (135, 206, 235, 255));
//! assert_eq!(c.to_hex_string(), "#87ceeb");
//! assert_eq!(c.to_rgb_string(), "rgb(135,206,235)");
//! # Ok(())
//! # }
//! ```
//!
//! ## Optional Features
//!
//! * `rust-rgb`: Enables converting from [`rgb`](https://crates.io/crates/rgb) crate types into `Color`.
//! * `cint`: Enables converting [`cint`](https://crates.io/crates/cint) crate types to and from `Color`.
//! * `serde`: Enables serializing (into HEX string) and deserializing (from any supported string color format) using [`serde`](https://serde.rs/) framework.

mod color;
mod parser;

pub use color::Color;
pub use parser::{parse, ParseColorError};
