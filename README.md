# CSS Color Parser for Rust

[![crates.io](https://img.shields.io/crates/v/csscolorparser.svg)](https://crates.io/crates/csscolorparser)
[![Documentation](https://docs.rs/csscolorparser/badge.svg)](https://docs.rs/csscolorparser)
[![Build Status](https://github.com/mazznoer/csscolorparser-rs/workflows/Rust/badge.svg)](https://github.com/mazznoer/csscolorparser-rs/actions)
[![Build Status](https://travis-ci.org/mazznoer/csscolorparser-rs.svg?branch=master)](https://travis-ci.org/mazznoer/csscolorparser-rs)
[![codecov](https://codecov.io/gh/mazznoer/csscolorparser-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mazznoer/csscolorparser-rs)

Rust CSS color parser.

It support W3C's CSS color module level 4.

## Usage

Add `csscolorparser` to your `Cargo.toml`

```
[dependencies]
csscolorparser = "0.2.0"
```

```rust
let c = csscolorparser::parse("rgb(100%,0%,0%)").unwrap();

assert_eq!(c.rgba(), (1.0, 0.0, 0.0, 1.0));
assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
assert_eq!(c.to_hex_string(), "#ff0000");
assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
```

## Supported Format

It support named colors, hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.

```text
--- example color format
transparent
gold
rebeccapurple
skyblue
lime
#0f0
#0f0f
#00ff00
#00ff00ff
rgb(0,255,0)
rgb(0% 100% 0%)
rgb(0 255 0 / 100%)
rgba(0,255,0,1)
hsl(120,100%,50%)
hsl(120deg 100% 50%)
hsl(-240 100% 50%)
hsl(-240deg 100% 50%)
hsl(0.3333turn 100% 50%)
hsl(133.333grad 100% 50%)
hsl(2.0944rad 100% 50%)
hsla(120,100%,50%,100%)
hwb(120 0% 0%)
hwb(480deg 0% 0% / 100%)
hsv(120,100%,100%)
hsv(120deg 100% 100% / 100%)
```

