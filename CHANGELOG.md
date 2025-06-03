# Changelog

## [Unreleased](https://github.com/mazznoer/csscolorparser-rs/compare/v0.7.2...HEAD)

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

