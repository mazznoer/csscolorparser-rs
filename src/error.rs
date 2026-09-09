use core::error::Error;
use core::fmt;

/// An error which can be returned when parsing a CSS color string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseColorError {
    /// Invalid hexadecimal format.
    InvalidHex,
    /// Invalid `rgb()` / `rgba()` format.
    InvalidRgb,
    /// Invalid `hsl()` / `hsla()` format.
    InvalidHsl,
    /// Invalid `hwb()` format.
    InvalidHwb,
    /// Invalid `hsv()` format.
    InvalidHsv,
    /// Invalid `lab()` format.
    InvalidLab,
    /// Invalid `lch()` format.
    InvalidLch,
    /// Invalid `oklab()` format.
    InvalidOklab,
    /// Invalid `oklch()` format.
    InvalidOklch,
    /// Invalid `color()` format.
    InvalidColor,
    /// Unrecognized color function.
    InvalidFunction,
    /// Unknown or unrecognized color format / named color.
    InvalidUnknown,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidHex => f.write_str("invalid hex format"),
            Self::InvalidRgb => f.write_str("invalid rgb format"),
            Self::InvalidHsl => f.write_str("invalid hsl format"),
            Self::InvalidHwb => f.write_str("invalid hwb format"),
            Self::InvalidHsv => f.write_str("invalid hsv format"),
            Self::InvalidLab => f.write_str("invalid lab format"),
            Self::InvalidLch => f.write_str("invalid lch format"),
            Self::InvalidOklab => f.write_str("invalid oklab format"),
            Self::InvalidOklch => f.write_str("invalid oklch format"),
            Self::InvalidColor => f.write_str("invalid color format"),
            Self::InvalidFunction => f.write_str("invalid color function"),
            Self::InvalidUnknown => f.write_str("invalid unknown format"),
        }
    }
}

impl Error for ParseColorError {}
