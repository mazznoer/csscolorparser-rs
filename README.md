<p align="center">
<strong>Rust CSS Color Parser Library</strong>
</p>

<p align="center">
<a href="https://github.com/mazznoer/csscolorparser-rs"><img alt="License" src="https://img.shields.io/crates/l/csscolorparser"></a>
<a href="https://crates.io/crates/csscolorparser"><img alt="crates.io" src="https://img.shields.io/crates/v/csscolorparser.svg"></a>
<a href="https://docs.rs/csscolorparser"><img alt="Documentation" src="https://docs.rs/csscolorparser/badge.svg"></a>
<a href="https://github.com/mazznoer/csscolorparser-rs/actions"><img alt="Build Status" src="https://github.com/mazznoer/csscolorparser-rs/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://crates.io/crates/csscolorparser"><img alt="Total Downloads" src="https://img.shields.io/crates/d/csscolorparser.svg"></a>
</p>

<p align="center">
    <strong>
        <a href="https://docs.rs/csscolorparser">Documentation</a> • <a href="CHANGELOG.md">Changelog</a> • <a href="#features">Features</a>
    </strong>
</p>

<hr>

[Rust](https://www.rust-lang.org/) library for parsing CSS color string as defined in the W3C's [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).

## Supported Color Format

* [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
* RGB hexadecimal (with and without `#` prefix)
     + Short format `#rgb`
     + Short format with alpha `#rgba`
     + Long format `#rrggbb`
     + Long format with alpha `#rrggbbaa`
* `rgb()` and `rgba()`
* `hsl()` and `hsla()`
* `hwb()`
* `lab()`
* `lch()`
* `oklab()`
* `oklch()`
* `hwba()`, `hsv()`, `hsva()` - not in CSS standard.

## Usage

Add this to your `Cargo.toml`

```toml
csscolorparser = "0.7"
```

## Examples

Using `csscolorparser::parse()` function.

```rust
let c = csscolorparser::parse("rgb(100%,0%,0%)")?;

assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
assert_eq!(c.to_css_hex(), "#ff0000");
assert_eq!(c.to_css_rgb(), "rgb(255 0 0)");
assert_eq!(c.name(), Some("red"));
```

Using `parse()` method on `&str`.

```rust
use csscolorparser::Color;

let c: Color = "#ff00007f".parse()?;

assert_eq!(c.to_rgba8(), [255, 0, 0, 127]);
assert_eq!(c.to_css_hex(), "#ff00007f");
```

## Features

### Default

* __named-colors__: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors). Requires [`phf`](https://crates.io/crates/phf). Can be disabled using `default-features = false`.

### Optional

* __lab__: Enables parsing `lab()` and `lch()` color format.
* __rust-rgb__: Enables converting from [`rgb`](https://crates.io/crates/rgb) crate types into `Color`.
* __cint__: Enables converting [`cint`](https://crates.io/crates/cint) crate types to and from `Color`.
* __serde__: Enables serializing (into HEX string) and deserializing (from any supported string color format) using [`serde`](https://serde.rs/) framework.

## Similar Projects

* [csscolorparser](https://github.com/mazznoer/csscolorparser) (Go)
* [csscolorparser](https://github.com/deanm/css-color-parser-js) (Javascript)

