use core::fmt;

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Strip prefix ignore case
pub(crate) fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.len() > s.len() {
        return None;
    }

    let (s_start, remainder) = s.split_at(prefix.len());

    if s_start.eq_ignore_ascii_case(prefix) {
        Some(remainder)
    } else {
        None
    }
}

// Strip suffix ignore case
pub(crate) fn strip_suffix<'a>(s: &'a str, suffix: &str) -> Option<&'a str> {
    let split_idx = s.len().checked_sub(suffix.len())?;

    let (prefix, s_end) = s.split_at(split_idx);

    if s_end.eq_ignore_ascii_case(suffix) {
        Some(prefix)
    } else {
        None
    }
}

#[inline]
fn parse_finite(s: &str) -> Option<f32> {
    s.parse::<f32>().ok().filter(|t| t.is_finite())
}

pub(crate) fn parse_percent_or_float(s: &str) -> Option<(f32, bool)> {
    if s.eq_ignore_ascii_case("none") {
        return Some((0.0, false));
    }

    if let Some(s) = s.strip_suffix('%') {
        parse_finite(s).map(|t| (t / 100.0, true))
    } else {
        parse_finite(s).map(|t| (t, false))
    }
}

pub(crate) fn parse_angle(s: &str) -> Option<f32> {
    if s.eq_ignore_ascii_case("none") {
        return Some(0.0);
    }

    if let Some(s) = strip_suffix(s, "deg") {
        return parse_finite(s);
    }

    if let Some(s) = strip_suffix(s, "grad") {
        return parse_finite(s).map(|t| t * 360.0 / 400.0);
    }

    if let Some(s) = strip_suffix(s, "rad") {
        return parse_finite(s).map(|t| t.to_degrees());
    }

    if let Some(s) = strip_suffix(s, "turn") {
        return parse_finite(s).map(|t| t * 360.0);
    }

    parse_finite(s)
}

// ---

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
        } else if fract % 10 != 0 {
            3
        } else if fract % 100 != 0 {
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

    #[test]
    fn strip_suffix_() {
        assert_eq!(strip_suffix("45deg", "deg"), Some("45"));
        assert_eq!(strip_suffix("90DEG", "deg"), Some("90"));
        assert_eq!(strip_suffix("0.25turn", "turn"), Some("0.25"));
        assert_eq!(strip_suffix("1.0Turn", "turn"), Some("1.0"));

        assert_eq!(strip_suffix("", "deg"), None);
        assert_eq!(strip_suffix("90", "deg"), None);
    }

    #[test]
    fn parse_percent_or_float_() {
        let test_data = [
            ("none", Some((0.0, false))),
            ("NONE", Some((0.0, false))),
            ("0%", Some((0.0, true))),
            ("100%", Some((1.0, true))),
            ("50%", Some((0.5, true))),
            ("0", Some((0.0, false))),
            ("1", Some((1.0, false))),
            ("0.5", Some((0.5, false))),
            ("100.0", Some((100.0, false))),
            ("-23.7", Some((-23.7, false))),
            ("%", None),
            ("1x", None),
            ("nan", None),
            ("inf", None),
            ("1e400", None),
            ("nan%", None),
            ("inf%", None),
            ("1e400%", None),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percent_or_float(s), expected);
        }
    }

    #[test]
    fn parse_angle_() {
        let test_data = [
            ("none", Some(0.0)),
            ("NONE", Some(0.0)),
            ("360", Some(360.0)),
            ("127.356", Some(127.356)),
            ("+120deg", Some(120.0)),
            ("90deg", Some(90.0)),
            ("-127deg", Some(-127.0)),
            ("100grad", Some(90.0)),
            ("1.5707963267948966rad", Some(90.0)),
            ("0.25turn", Some(90.0)),
            ("-0.25turn", Some(-90.0)),
            ("O", None),
            ("Odeg", None),
            ("rad", None),
            ("nan", None),
            ("inf", None),
            ("1e400", None),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_angle(s), expected);
        }
    }
}
