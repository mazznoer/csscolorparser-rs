use crate::utils::ParamParser;
use crate::utils::parse_values;
use crate::utils::remap;
use crate::{Color, ParseColorError};

#[cfg(feature = "named-colors")]
use crate::NAMED_COLORS;

/// Parse CSS color string
///
/// # Examples
///
/// ```
/// # use core::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("#ff0")?;
///
/// assert_eq!(c.to_array(), [1.0, 1.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 255, 0, 255]);
/// assert_eq!(c.to_css_hex().to_string(), "#ffff00");
/// assert_eq!(c.to_css_rgb().to_string(), "rgb(255 255 0)");
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use core::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)")?;
///
/// assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
/// assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
/// assert_eq!(c.to_css_hex().to_string(), "#ff0000");
/// assert_eq!(c.to_css_rgb().to_string(), "rgb(255 0 0)");
/// # Ok(())
/// # }
/// ```
#[inline(never)]
pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    let s = s.trim();

    let err = match parse_abs(s) {
        Ok(c) => return Ok(c),
        Err(e @ ParseColorError::InvalidHex) => return Err(e),
        Err(e @ ParseColorError::InvalidFunction) => return Err(e),
        Err(e @ ParseColorError::InvalidUnknown) => return Err(e),
        Err(e) => e,
    };

    if let (Some(idx), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        if !s.is_ascii() {
            return Err(err);
        }

        let mut pp = ParamParser::new(&s[idx + 1..]);
        pp.space();

        #[rustfmt::skip]
        let (
            Some(from),  true,
            Some(color), true,
            Some(val1),  true,
            Some(val2),  true,
            Some(val3),
        ) = (
            pp.value(), pp.space(),
            pp.value(), pp.space(),
            pp.value(), pp.space(),
            pp.value(), pp.space(),
            pp.value(),
        ) else {
            return Err(err);
        };

        if !from.eq_ignore_ascii_case("from") {
            return Err(err);
        }

        let Ok(color) = parse(color) else {
            return Err(err);
        };

        pp.space();

        let val4 = if pp.is_end() {
            "alpha"
        } else if let (true, Some(alpha), _, true) =
            (pp.slash(), pp.value(), pp.space(), pp.is_end())
        {
            alpha
        } else {
            return Err(err);
        };

        let values = [val1, val2, val3, val4];

        match err {
            ParseColorError::InvalidRgb => {
                // r, g, b [0..255]
                // alpha   [0..1]
                let variables = [
                    ("r", color.r * 255.0),
                    ("g", color.g * 255.0),
                    ("b", color.b * 255.0),
                    ("alpha", color.a),
                ];
                if let Some([r, g, b, a]) = parse_values(values, variables) {
                    return Ok(Color::new(r / 255.0, g / 255.0, b / 255.0, a));
                };
            }
            ParseColorError::InvalidHwb => {
                // h     [0..360]
                // w, b  [0..100]
                // alpha [0..1]
                let [h, w, b, a] = color.to_hwba();
                let variables = [("h", h), ("w", w * 100.0), ("b", b * 100.0), ("alpha", a)];
                if let Some([h, w, b, a]) = parse_values(values, variables) {
                    return Ok(Color::from_hwba(h, w / 100.0, b / 100.0, a));
                };
            }
            ParseColorError::InvalidHsl => {
                // h     [0..360]
                // s, l  [0..100]
                // alpha [0..1]
                let [h, s, l, a] = color.to_hsla();
                let variables = [("h", h), ("s", s * 100.0), ("l", l * 100.0), ("alpha", a)];
                if let Some([h, s, l, a]) = parse_values(values, variables) {
                    return Ok(Color::from_hsla(
                        h,
                        (s / 100.0).clamp(0.0, 1.0),
                        (l / 100.0).clamp(0.0, 1.0),
                        a,
                    ));
                };
            }
            ParseColorError::InvalidHsv => {
                // h     [0..360]
                // s, v  [0..100]
                // alpha [0..1]
                let [h, s, v, a] = color.to_hsva();
                let variables = [("h", h), ("s", s * 100.0), ("v", v * 100.0), ("alpha", a)];
                if let Some([h, s, v, a]) = parse_values(values, variables) {
                    return Ok(Color::from_hsva(h, s / 100.0, v / 100.0, a));
                };
            }
            ParseColorError::InvalidLab => {
                // l     [0..100]
                // a, b  [-125..125]
                // alpha [0..1]
                let [l, a, b, alpha] = color.to_laba();
                let variables = [("l", l), ("a", a), ("b", b), ("alpha", alpha)];
                if let Some([l, a, b, alpha]) = parse_values(values, variables) {
                    return Ok(Color::from_laba(l.max(0.0), a, b, alpha));
                };
            }
            ParseColorError::InvalidLch => {
                // l [0..100]
                // c [0..150]
                // h [0..360]
                // alpha [0..1]
                let [l, c, h, a] = color.to_lcha();
                let variables = [("l", l), ("c", c), ("h", h.to_degrees()), ("alpha", a)];
                if let Some([l, c, h, a]) = parse_values(values, variables) {
                    return Ok(Color::from_lcha(l.max(0.0), c.max(0.0), h.to_radians(), a));
                };
            }
            ParseColorError::InvalidOklab => {
                // l     [0..1]
                // a, b  [-0.4 .. 0.4]
                // alpha [0..1]
                let [l, a, b, alpha] = color.to_oklaba();
                let variables = [("l", l), ("a", a), ("b", b), ("alpha", alpha)];
                if let Some([l, a, b, alpha]) = parse_values(values, variables) {
                    return Ok(Color::from_oklaba(l.max(0.0), a, b, alpha));
                };
            }
            ParseColorError::InvalidOklch => {
                // l [0..1]
                // c [0..0.4]
                // h [0..360]
                // alpha [0..1]
                let [l, c, h, a] = color.to_oklcha();
                let variables = [("l", l), ("c", c), ("h", h.to_degrees()), ("alpha", a)];
                if let Some([l, c, h, a]) = parse_values(values, variables) {
                    return Ok(Color::from_oklcha(
                        l.max(0.0),
                        c.max(0.0),
                        h.to_radians(),
                        a,
                    ));
                };
            }
            _ => unreachable!(),
        }
        return Err(err);
    }

    unreachable!();
}

fn parse_abs(s: &str) -> Result<Color, ParseColorError> {
    if s.eq_ignore_ascii_case("transparent") {
        return Ok(Color::new(0.0, 0.0, 0.0, 0.0));
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        return parse_hex(s);
    }

    if let (Some(idx), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..idx].trim_end();

        let err = match fname {
            s if s.eq_ignore_ascii_case("rgb") || s.eq_ignore_ascii_case("rgba") => {
                ParseColorError::InvalidRgb
            }
            s if s.eq_ignore_ascii_case("hsl") || s.eq_ignore_ascii_case("hsla") => {
                ParseColorError::InvalidHsl
            }
            s if s.eq_ignore_ascii_case("hwb") || s.eq_ignore_ascii_case("hwba") => {
                ParseColorError::InvalidHwb
            }
            s if s.eq_ignore_ascii_case("hsv") || s.eq_ignore_ascii_case("hsva") => {
                ParseColorError::InvalidHsv
            }
            s if s.eq_ignore_ascii_case("lab") => ParseColorError::InvalidLab,
            s if s.eq_ignore_ascii_case("lch") => ParseColorError::InvalidLch,
            s if s.eq_ignore_ascii_case("oklab") => ParseColorError::InvalidOklab,
            s if s.eq_ignore_ascii_case("oklch") => ParseColorError::InvalidOklch,
            _ => return Err(ParseColorError::InvalidFunction),
        };

        let s = &s[idx + 1..];

        if !s.is_ascii() {
            return Err(err);
        }

        let mut pp = ParamParser::new(s);
        pp.space();

        let (Some(val0), true, Some(val1), true, Some(val2)) = (
            pp.value(),
            pp.comma_or_space(),
            pp.value(),
            pp.comma_or_space(),
            pp.value(),
        ) else {
            return Err(err);
        };

        let is_space = pp.space();

        let alpha = if pp.is_end() {
            1.0
        } else if let (true, Some(a), _, true) = (
            pp.comma_or_slash() || is_space,
            pp.value(),
            pp.space(),
            pp.is_end(),
        ) {
            if let Some((v, _)) = parse_percent_or_float(a) {
                v.clamp(0.0, 1.0)
            } else {
                return Err(err);
            }
        } else {
            return Err(err);
        };

        match err {
            ParseColorError::InvalidRgb => {
                if let (Some(r), Some(g), Some(b)) = (
                    // red
                    parse_percent_or_255(val0),
                    // green
                    parse_percent_or_255(val1),
                    // blue
                    parse_percent_or_255(val2),
                ) {
                    return Ok(Color {
                        r: r.clamp(0.0, 1.0),
                        g: g.clamp(0.0, 1.0),
                        b: b.clamp(0.0, 1.0),
                        a: alpha,
                    });
                }
            }
            ParseColorError::InvalidHsl => {
                if let (Some(h), Some((s, _)), Some((l, _))) = (
                    // hue
                    parse_angle(val0),
                    // saturation
                    parse_percent_or_float(val1),
                    // lightness
                    parse_percent_or_float(val2),
                ) {
                    return Ok(Color::from_hsla(h, s, l, alpha));
                }
            }
            ParseColorError::InvalidHwb => {
                if let (Some(h), Some((w, _)), Some((b, _))) = (
                    // hue
                    parse_angle(val0),
                    // whiteness
                    parse_percent_or_float(val1),
                    // blackness
                    parse_percent_or_float(val2),
                ) {
                    return Ok(Color::from_hwba(h, w, b, alpha));
                }
            }
            ParseColorError::InvalidHsv => {
                if let (Some(h), Some((s, _)), Some((v, _))) = (
                    // hue
                    parse_angle(val0),
                    // saturation
                    parse_percent_or_float(val1),
                    // value
                    parse_percent_or_float(val2),
                ) {
                    return Ok(Color::from_hsva(h, s, v, alpha));
                }
            }
            ParseColorError::InvalidLab => {
                if let (Some((l, l_pct)), Some((a, a_pct)), Some((b, b_pct))) = (
                    // lightness
                    parse_percent_or_float(val0),
                    // a
                    parse_percent_or_float(val1),
                    // b
                    parse_percent_or_float(val2),
                ) {
                    let l = if l_pct { l * 100.0 } else { l };
                    let a = if a_pct {
                        remap(a, -1.0, 1.0, -125.0, 125.0)
                    } else {
                        a
                    };
                    let b = if b_pct {
                        remap(b, -1.0, 1.0, -125.0, 125.0)
                    } else {
                        b
                    };
                    return Ok(Color::from_laba(l.max(0.0), a, b, alpha));
                }
            }
            ParseColorError::InvalidLch => {
                if let (Some((l, l_pct)), Some((c, c_pct)), Some(h)) = (
                    // lightness
                    parse_percent_or_float(val0),
                    // chroma
                    parse_percent_or_float(val1),
                    // hue
                    parse_angle(val2),
                ) {
                    let l = if l_pct { l * 100.0 } else { l };
                    let c = if c_pct { c * 150.0 } else { c };
                    return Ok(Color::from_lcha(
                        l.max(0.0),
                        c.max(0.0),
                        h.to_radians(),
                        alpha,
                    ));
                }
            }
            ParseColorError::InvalidOklab => {
                if let (Some((l, _)), Some((a, a_pct)), Some((b, b_pct))) = (
                    // lightness
                    parse_percent_or_float(val0),
                    // a
                    parse_percent_or_float(val1),
                    // b
                    parse_percent_or_float(val2),
                ) {
                    let a = if a_pct {
                        remap(a, -1.0, 1.0, -0.4, 0.4)
                    } else {
                        a
                    };
                    let b = if b_pct {
                        remap(b, -1.0, 1.0, -0.4, 0.4)
                    } else {
                        b
                    };
                    return Ok(Color::from_oklaba(l.max(0.0), a, b, alpha));
                }
            }
            ParseColorError::InvalidOklch => {
                if let (Some((l, _)), Some((c, c_pct)), Some(h)) = (
                    // lightness
                    parse_percent_or_float(val0),
                    // chroma
                    parse_percent_or_float(val1),
                    // hue
                    parse_angle(val2),
                ) {
                    let c = if c_pct { c * 0.4 } else { c };
                    return Ok(Color::from_oklcha(
                        l.max(0.0),
                        c.max(0.0),
                        h.to_radians(),
                        alpha,
                    ));
                }
            }
            _ => unreachable!(),
        }
        return Err(err);
    }

    // Hex format without prefix '#'
    if let Ok(c) = parse_hex(s) {
        return Ok(c);
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if s.len() > 2
        && s.len() < 21
        && let Some([r, g, b]) = NAMED_COLORS.get(s.into())
    {
        return Ok(Color::from_rgba8(*r, *g, *b, 255));
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
    if s.eq_ignore_ascii_case("none") {
        return Some((0.0, false));
    }
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
        .or_else(|| s.parse().ok().map(|t| (t, false)))
}

fn parse_percent_or_255(s: &str) -> Option<f32> {
    if s.eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| t / 100.0))
        .or_else(|| s.parse().ok().map(|t: f32| t / 255.0))
}

fn parse_angle(s: &str) -> Option<f32> {
    if s.eq_ignore_ascii_case("none") {
        return Some(0.0);
    }
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
mod t {
    use super::*;

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
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percent_or_float(s), expected);
        }
    }

    #[test]
    fn parse_percent_or_255_() {
        let test_data = [
            ("none", Some(0.0)),
            ("NONE", Some(0.0)),
            ("0%", Some(0.0)),
            ("100%", Some(1.0)),
            ("50%", Some(0.5)),
            ("-100%", Some(-1.0)),
            ("0", Some(0.0)),
            ("255", Some(1.0)),
            ("127.5", Some(0.5)),
            ("%", None),
            ("255x", None),
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_percent_or_255(s), expected);
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
        ];
        for (s, expected) in test_data {
            assert_eq!(parse_angle(s), expected);
        }
    }

    #[test]
    fn parse_hex_() {
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
