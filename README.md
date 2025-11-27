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

### Absolute Color

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

### Relative Color

Example:

```text
rgb(from red r g calc(b + 20))
rgb(from gold calc(((r + g) + b) / 3) 127 127)
hwb(from #bad455 calc(h + 35) w b)
hsl(from purple h s l / 0.5)
```

#### Relative Color Format Limitations

Doesn't support percentage.

`calc()` only support the following expression:

```[OPERAND] [OPERATOR] [OPERAND]```

`OPERATOR` is one of `+`, `-`, `*` or `/`
`OPERAND` can be a number, a variable (`r`, `g`, `b`, `alpha` etc. depends on color function) or another expression wrapped in parenthesis.

##### Not Supported

```
rgb(from #bad455 100% g b)
rgb(from #bad455 r g b / 50%)
rgb(from #bad455 calc(r+g-30) 90 b)
```

##### OK

```
rgb(from #bad455 255 g b)
rgb(from #bad455 r g b / 0.5)
rgb(from #bad455 calc(r+15) 90 b)
rgb(from #bad455 calc((r+g)-30) 90 b)
hwb(from rgb(from rgb(100% 0% 50%) r g 75) calc(h+25) w b)
```

## Usage

Add this to your `Cargo.toml`

```toml
csscolorparser = "0.8"
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

* __std__: Using the standard library.
* __named-colors__: Enables parsing from [named colors](https://www.w3.org/TR/css-color-4/#named-colors).

Default features can be disabled using `default-features = false`.

### Optional

* __rust-rgb__: Enables converting from [`rgb`](https://crates.io/crates/rgb) crate types into `Color`.
* __cint__: Enables converting [`cint`](https://crates.io/crates/cint) crate types to and from `Color`.
* __serde__: Enables serializing (into HEX string) and deserializing (from any supported string color format) using [`serde`](https://serde.rs/) framework.

## Similar Projects

* [csscolorparser](https://github.com/mazznoer/csscolorparser) (Go)
* [csscolorparser](https://github.com/deanm/css-color-parser-js) (Javascript)

