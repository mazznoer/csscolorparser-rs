use std::{error, fmt};

use crate::utils::remap;
use crate::Color;

#[cfg(feature = "named-colors")]
use crate::NAMED_COLORS;

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
    #[cfg(feature = "lab")]
    InvalidLab,
    /// A CSS color string was invalid lch format.
    #[cfg(feature = "lab")]
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
            #[cfg(feature = "lab")]
            Self::InvalidLab => f.write_str("invalid lab format"),
            #[cfg(feature = "lab")]
            Self::InvalidLch => f.write_str("invalid lch format"),
            Self::InvalidOklab => f.write_str("invalid oklab format"),
            Self::InvalidOklch => f.write_str("invalid oklch format"),
            Self::InvalidFunction => f.write_str("invalid color function"),
            Self::InvalidUnknown => f.write_str("invalid unknown format"),
        }
    }
}

impl error::Error for ParseColorError {}

/// Parse CSS color string
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("#ff0")?;
///
/// assert_eq!(c.to_array(), [1.0, 1.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 255, 0, 255]);
/// assert_eq!(c.to_css_hex(), "#ffff00");
/// assert_eq!(c.to_css_rgb(), "rgb(255 255 0)");
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)")?;
///
/// assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
/// assert_eq!(c.to_css_hex(), "#ff0000");
/// assert_eq!(c.to_css_rgb(), "rgb(255 0 0)");
/// # Ok(())
/// # }
/// ```
#[inline(never)]
pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    let s = s.trim();

    if s.eq_ignore_ascii_case("transparent") {
        return Ok(Color::new(0.0, 0.0, 0.0, 0.0));
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        return parse_hex(s);
    }

    if let (Some(idx), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..idx].trim_end();
        let mut params = s[idx + 1..]
            .split(&[',', '/'])
            .flat_map(str::split_ascii_whitespace);

        let (Some(val0), Some(val1), Some(val2)) = (params.next(), params.next(), params.next())
        else {
            return Err(ParseColorError::InvalidFunction);
        };

        let alpha = if let Some(a) = params.next() {
            if let Some((v, _)) = parse_percent_or_float(a) {
                v.clamp(0.0, 1.0)
            } else {
                return Err(ParseColorError::InvalidFunction);
            }
        } else {
            1.0
        };

        if params.next().is_some() {
            return Err(ParseColorError::InvalidFunction);
        }

        if fname.eq_ignore_ascii_case("rgb") || fname.eq_ignore_ascii_case("rgba") {
            if let (Some((r, r_fmt)), Some((g, g_fmt)), Some((b, b_fmt))) = (
                // red
                parse_percent_or_255(val0),
                // green
                parse_percent_or_255(val1),
                // blue
                parse_percent_or_255(val2),
            ) {
                if r_fmt == g_fmt && g_fmt == b_fmt {
                    return Ok(Color {
                        r: r.clamp(0.0, 1.0),
                        g: g.clamp(0.0, 1.0),
                        b: b.clamp(0.0, 1.0),
                        a: alpha,
                    });
                }
            }

            return Err(ParseColorError::InvalidRgb);
        } else if fname.eq_ignore_ascii_case("hsl") || fname.eq_ignore_ascii_case("hsla") {
            if let (Some(h), Some((s, s_fmt)), Some((l, l_fmt))) = (
                // hue
                parse_angle(val0),
                // saturation
                parse_percent_or_float(val1),
                // lightness
                parse_percent_or_float(val2),
            ) {
                if s_fmt == l_fmt {
                    return Ok(Color::from_hsla(h, s, l, alpha));
                }
            }

            return Err(ParseColorError::InvalidHsl);
        } else if fname.eq_ignore_ascii_case("hwb") || fname.eq_ignore_ascii_case("hwba") {
            if let (Some(h), Some((w, w_fmt)), Some((b, b_fmt))) = (
                // hue
                parse_angle(val0),
                // whiteness
                parse_percent_or_float(val1),
                // blackness
                parse_percent_or_float(val2),
            ) {
                if w_fmt == b_fmt {
                    return Ok(Color::from_hwba(h, w, b, alpha));
                }
            }

            return Err(ParseColorError::InvalidHwb);
        } else if fname.eq_ignore_ascii_case("hsv") || fname.eq_ignore_ascii_case("hsva") {
            if let (Some(h), Some((s, s_fmt)), Some((v, v_fmt))) = (
                // hue
                parse_angle(val0),
                // saturation
                parse_percent_or_float(val1),
                // value
                parse_percent_or_float(val2),
            ) {
                if s_fmt == v_fmt {
                    return Ok(Color::from_hsva(h, s, v, alpha));
                }
            }

            return Err(ParseColorError::InvalidHsv);
        } else if fname.eq_ignore_ascii_case("lab") {
            #[cfg(feature = "lab")]
            if let (Some((l, l_fmt)), Some((a, a_fmt)), Some((b, b_fmt))) = (
                // lightness
                parse_percent_or_float(val0),
                // a
                parse_percent_or_float(val1),
                // b
                parse_percent_or_float(val2),
            ) {
                let l = if l_fmt { l * 100.0 } else { l };
                let a = if a_fmt {
                    remap(a, -1.0, 1.0, -125.0, 125.0)
                } else {
                    a
                };
                let b = if b_fmt {
                    remap(b, -1.0, 1.0, -125.0, 125.0)
                } else {
                    b
                };
                return Ok(Color::from_laba(l.max(0.0), a, b, alpha));
            } else {
                return Err(ParseColorError::InvalidLab);
            }
        } else if fname.eq_ignore_ascii_case("lch") {
            #[cfg(feature = "lab")]
            if let (Some((l, l_fmt)), Some((c, c_fmt)), Some(h)) = (
                // lightness
                parse_percent_or_float(val0),
                // chroma
                parse_percent_or_float(val1),
                // hue
                parse_angle(val2),
            ) {
                let l = if l_fmt { l * 100.0 } else { l };
                let c = if c_fmt { c * 150.0 } else { c };
                return Ok(Color::from_lcha(
                    l.max(0.0),
                    c.max(0.0),
                    h.to_radians(),
                    alpha,
                ));
            } else {
                return Err(ParseColorError::InvalidLch);
            }
        } else if fname.eq_ignore_ascii_case("oklab") {
            if let (Some((l, _)), Some((a, a_fmt)), Some((b, b_fmt))) = (
                // lightness
                parse_percent_or_float(val0),
                // a
                parse_percent_or_float(val1),
                // b
                parse_percent_or_float(val2),
            ) {
                let a = if a_fmt {
                    remap(a, -1.0, 1.0, -0.4, 0.4)
                } else {
                    a
                };
                let b = if b_fmt {
                    remap(b, -1.0, 1.0, -0.4, 0.4)
                } else {
                    b
                };
                return Ok(Color::from_oklaba(l.max(0.0), a, b, alpha));
            }

            return Err(ParseColorError::InvalidOklab);
        } else if fname.eq_ignore_ascii_case("oklch") {
            if let (Some((l, _)), Some((c, c_fmt)), Some(h)) = (
                // lightness
                parse_percent_or_float(val0),
                // chroma
                parse_percent_or_float(val1),
                // hue
                parse_angle(val2),
            ) {
                let c = if c_fmt { c * 0.4 } else { c };
                return Ok(Color::from_oklcha(
                    l.max(0.0),
                    c.max(0.0),
                    h.to_radians(),
                    alpha,
                ));
            }

            return Err(ParseColorError::InvalidOklch);
        }

        return Err(ParseColorError::InvalidFunction);
    }

    // Hex format without prefix '#'
    if let Ok(c) = parse_hex(s) {
        return Ok(c);
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if s.len() > 2 && s.len() < 21 {
        let s = s.to_ascii_lowercase();
        if let Some([r, g, b]) = NAMED_COLORS.get(&s) {
            return Ok(Color::from_rgba8(*r, *g, *b, 255));
        }
    }

    Err(ParseColorError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<Color, ParseColorError> {
    if !s.is_ascii() {
        return Err(ParseColorError::InvalidHex);
    }

    let n = s.len();

    fn parse_single_digit(digit: &str) -> Result<u8, ParseColorError> {
        u8::from_str_radix(digit, 16)
            .map(|n| (n << 4) | n)
            .map_err(|_| ParseColorError::InvalidHex)
    }

    if n == 3 || n == 4 {
        let r = parse_single_digit(&s[0..1])?;
        let g = parse_single_digit(&s[1..2])?;
        let b = parse_single_digit(&s[2..3])?;

        let a = if n == 4 {
            parse_single_digit(&s[3..4])?
        } else {
            255
        };

        Ok(Color::from_rgba8(r, g, b, a))
    } else if n == 6 || n == 8 {
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| ParseColorError::InvalidHex)?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| ParseColorError::InvalidHex)?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| ParseColorError::InvalidHex)?;

        let a = if n == 8 {
            u8::from_str_radix(&s[6..8], 16).map_err(|_| ParseColorError::InvalidHex)?
        } else {
            255
        };

        Ok(Color::from_rgba8(r, g, b, a))
    } else {
        Err(ParseColorError::InvalidHex)
    }
}

// strip suffix ignore case
fn strip_suffix<'a>(s: &'a str, suffix: &str) -> Option<&'a str> {
    if suffix.len() > s.len() {
        return None;
    }
    let s_end = &s[s.len() - suffix.len()..];
    if s_end.eq_ignore_ascii_case(suffix) {
        Some(&s[..s.len() - suffix.len()])
    } else {
        None
    }
}

fn parse_percent_or_float(s: &str) -> Option<(f32, bool)> {
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
        .or_else(|| s.parse().ok().map(|t| (t, false)))
}

fn parse_percent_or_255(s: &str) -> Option<(f32, bool)> {
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
        .or_else(|| s.parse().ok().map(|t: f32| (t / 255.0, false)))
}

fn parse_angle(s: &str) -> Option<f32> {
    strip_suffix(s, "deg")
        .and_then(|s| s.parse().ok())
        .or_else(|| {
            strip_suffix(s, "grad")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t * 360.0 / 400.0)
        })
        .or_else(|| {
            strip_suffix(s, "rad")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t.to_degrees())
        })
        .or_else(|| {
            strip_suffix(s, "turn")
                .and_then(|s| s.parse().ok())
                .map(|t: f32| t * 360.0)
        })
        .or_else(|| s.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_suffix() {
        assert_eq!(strip_suffix("45deg", "deg"), Some("45"));
        assert_eq!(strip_suffix("90DEG", "deg"), Some("90"));
        assert_eq!(strip_suffix("0.25turn", "turn"), Some("0.25"));
        assert_eq!(strip_suffix("1.0Turn", "turn"), Some("1.0"));

        assert_eq!(strip_suffix("", "deg"), None);
        assert_eq!(strip_suffix("90", "deg"), None);
    }

    #[test]
    fn test_parse_percent_or_float() {
        let test_data = [
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
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percent_or_float(s), expected);
        }
    }

    #[test]
    fn test_parse_percent_or_255() {
        let test_data = [
            ("0%", Some((0.0, true))),
            ("100%", Some((1.0, true))),
            ("50%", Some((0.5, true))),
            ("-100%", Some((-1.0, true))),
            ("0", Some((0.0, false))),
            ("255", Some((1.0, false))),
            ("127.5", Some((0.5, false))),
            ("%", None),
            ("255x", None),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percent_or_255(s), expected);
        }
    }

    #[test]
    fn test_parse_angle() {
        let test_data = [
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
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_angle(s), expected);
        }
    }

    #[test]
    fn test_parse_hex() {
        // case-insensitive tests
        macro_rules! cmp {
            ($a:expr, $b:expr) => {
                assert_eq!(
                    parse_hex($a).unwrap().to_rgba8(),
                    parse_hex($b).unwrap().to_rgba8()
                );
            };
        }
        cmp!("abc", "ABC");
        cmp!("DeF", "dEf");
        cmp!("f0eB", "F0Eb");
        cmp!("abcdef", "ABCDEF");
        cmp!("Ff03E0cB", "fF03e0Cb");
    }
}
