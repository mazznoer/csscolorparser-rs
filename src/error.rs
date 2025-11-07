use core::error::Error;
use core::fmt;

/// An error which can be returned when parsing a CSS color string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseColorError {
    /// A CSS color string was invalid hex format.
    InvalidHex,
    /// A CSS color string was invalid rgb format.
    InvalidRgb,
    /// A CSS color string was invalid hsl format.
    InvalidHsl,
    /// A CSS color string was invalid hwb format.
    InvalidHwb,
    /// A CSS color string was invalid hsv format.
    InvalidHsv,
    /// A CSS color string was invalid lab format.
    InvalidLab,
    /// A CSS color string was invalid lch format.
    InvalidLch,
    /// A CSS color string was invalid oklab format.
    InvalidOklab,
    /// A CSS color string was invalid oklch format.
    InvalidOklch,
    /// A CSS color string was invalid color function.
    InvalidFunction,
    /// A CSS color string was invalid unknown format.
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
            Self::InvalidFunction => f.write_str("invalid color function"),
            Self::InvalidUnknown => f.write_str("invalid unknown format"),
        }
    }
}

impl Error for ParseColorError {}
