use core::fmt;

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Strip prefix ignore case.
pub fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.len() > s.len() {
        return None;
    }
    let s_start = &s[..prefix.len()];
    if s_start.eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

pub(crate) struct AlphaFmt(pub f32);

impl fmt::Display for AlphaFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = (self.0.clamp(0.0, 1.0) * 100.0 + 0.5) as u8;
        if t < 100 {
            write!(f, " / {t}%")
        } else {
            Ok(())
        }
    }
}

pub(crate) struct FloatFmt(pub f32);

impl fmt::Display for FloatFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_nan() {
            return write!(f, "none");
        }

        // Round to 3 decimal places first to clear floating point noise
        let rounded = (self.0 * 1000.0).round() / 1000.0;

        // Get the 3-digit fractional part as an integer
        let fract = (rounded.abs().fract() * 1000.0).round() as u16;

        // Determine precision based on the integer fractional part
        let precision = if fract == 0 {
            0
        } else if !fract.is_multiple_of(10) {
            3
        } else if !fract.is_multiple_of(100) {
            2
        } else {
            1
        };

        write!(f, "{:.*}", precision, rounded)
    }
}

pub(crate) struct OpaqueDisplay<F>(pub F);

impl<F> fmt::Display for OpaqueDisplay<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

impl<F> fmt::Debug for OpaqueDisplay<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("\"{}\"", self))
    }
}

macro_rules! opaque_display {
    ($($arg:tt)*) => {
        $crate::utils::OpaqueDisplay(move |f: &mut ::core::fmt::Formatter<'_>| -> ::core::fmt::Result {
            f.write_fmt(format_args!($($arg)*))
        })
    };
}

pub(crate) use opaque_display;

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn strip_prefix_() {
        assert_eq!(strip_prefix("rgb(77)", "rgb"), Some("(77)"));
        assert_eq!(strip_prefix("RGB(0,0)", "rgb"), Some("(0,0)"));
        assert_eq!(strip_prefix("Hsv()", "HSV"), Some("()"));

        assert_eq!(strip_prefix("", "rgb"), None);
        assert_eq!(strip_prefix("10", "rgb"), None);
        assert_eq!(strip_prefix("hsv(0,0)", "hsva"), None);
        assert_eq!(strip_prefix("hsv", "hsva"), None);
    }
}
