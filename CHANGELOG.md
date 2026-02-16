# Changelog

## [Unreleased](https://github.com/mazznoer/csscolorparser-rs/compare/v0.8.1...HEAD)

### Added

- `parse_colors()` for parsing string containing multiple colors separated by comma.

### Changed

- `Color.to_css_*()` return `impl fmt::Display + fmt::Debug`
- Deprecate `Color.to_linear_rgba_u8()`
- Set Rust edition to 2024

### Removed

- Old deprecated methods.

## [0.8.2]

### Changed

- Parse `none` value in absolute color format.
- Changed `impl Display` for `Color` to hex color format.
- Remove unnecessary allocations.

## [0.8.1](https://github.com/mazznoer/csscolorparser-rs/compare/v0.8.0...v0.8.1)

### Changed

- Improvements in parser code.
- improvements of `calc()` parser for relative color format.

### Fixed

- Require `phf` only if needed.

## [0.8.0](https://github.com/mazznoer/csscolorparser-rs/compare/v0.7.2...v0.8.0)

### Added

- Support `no_std`.
- Support parsing relative color format.

### Changed

- Support for parsing lab format is always enabled now. Remove the `lab` cargo feature.
- Using `phf::OrderedMap` and `uncased` to store named colors.

## [0.7.2](https://github.com/mazznoer/csscolorparser-rs/compare/v0.7.1...v0.7.2)

### Added

- `Color::to_oklcha()`
- `Color::to_css_hex()`
- `Color::to_css_rgb()`
- `Color::to_css_hsl()`
- `Color::to_css_hwb()`
- `Color::to_css_lab()`
- `Color::to_css_lch()`
- `Color::to_css_oklab()`
- `Color::to_css_oklch()`

### Changed

- Deprecate `Color::to_hex_string()` and `Color::to_rgb_string()`

## [0.7.1](https://github.com/mazznoer/csscolorparser-rs/compare/v0.7.0...v0.7.1)

### Changed

- Remove some unnecessary allocations on parser code.

## [0.7.0](https://github.com/mazznoer/csscolorparser-rs/compare/v0.6.2...v0.7.0)

### Added

- `Color::from_oklcha()`
- Support parsing `oklab()` and `oklch()` color format.
- `Color::{from,to}_{laba,lcha}()`

### Changed

- `f64` -> `f32`
- Return type for `Color::to_{hsva,hsla,hwba,lab,lch,oklaba,linear_rgba}()` changed from tuple to array.
- Deprecate `Color::{from,to}_{lab,lch}()`, use `Color::{from,to}_{laba,lcha}()` instead.
- `NAMED_COLORS` is now public

### Removed

### Fixed

- Fix parsing `lab()` and `lch()` color format.
- Update `oklab` formula.

