//! # Overview
//!
//! Rust library for parsing CSS color string as defined in the W3C's [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).
//!
//! ## Supported Color Format
//!
//! * [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
//! * RGB hexadecimal (with and without `#` prefix)
//!      + Short format `#rgb`
//!      + Short format with alpha `#rgba`
//!      + Long format `#rrggbb`
//!      + Long format with alpha `#rrggbbaa`
//! * `rgb()` and `rgba()`
//! * `hsl()` and `hsla()`
//! * `hwb()`
//! * `lab()`
//! * `lch()`
//! * `oklab()`
//! * `oklch()`
//! * `hwba()`, `hsv()`, `hsva()` - not in CSS standard.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//!
//! ```toml
//! csscolorparser = "0.7"
//! ```
//!
//! ## Examples
//!
//! Using [`csscolorparser::parse()`](fn.parse.html) function.
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let c = csscolorparser::parse("rgb(100%,0%,0%)")?;
//!
//! assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
//! assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
//! assert_eq!(c.to_css_hex(), "#ff0000");
//! assert_eq!(c.to_css_rgb(), "rgb(255 0 0)");
//! # Ok(())
//! # }
//! ```
//!
//! Using `parse()` method on `&str`.
//!
//! ```rust
//! use csscolorparser::Color;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//! let c: Color = "#ff00007f".parse()?;
//!
//! assert_eq!(c.to_rgba8(), [255, 0, 0, 127]);
//! assert_eq!(c.to_css_hex(), "#ff00007f");
//! # Ok(())
//! # }
//! ```
//!
//! ## Default Feature
//!
//! * `named-colors`: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors). Requires [`phf`](https://crates.io/crates/phf).
//!
//! ## Optional Features
//!
//! * `lab`: Enables parsing `lab()` and `lch()` color format.
//! * `rust-rgb`: Enables converting from [`rgb`](https://crates.io/crates/rgb) crate types into `Color`.
//! * `cint`: Enables converting [`cint`](https://crates.io/crates/cint) crate types to and from `Color`.
//! * `serde`: Enables serializing (into HEX string) and deserializing (from any supported string color format) using [`serde`](https://serde.rs/) framework.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color;
mod color2;
pub use color::Color;

mod parser;
pub use parser::{parse, ParseColorError};

#[cfg(feature = "named-colors")]
mod named_colors;
#[cfg(feature = "named-colors")]
pub use named_colors::NAMED_COLORS;

#[cfg(feature = "cint")]
mod cint;

#[cfg(feature = "lab")]
mod lab;

mod utils;
