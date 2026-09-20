use crate::utils::ParamParser;
use crate::utils::parse_values;
use crate::utils::remap;
use crate::utils::{parse_angle, parse_percent_or_255, parse_percent_or_float};
use crate::{Color, ParseColorError};

#[cfg(feature = "named-colors")]
use crate::NAMED_COLORS;

const MAX_DEPTH: usize = 32;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ColorFunc {
    Srgb,
    SrgbLinear,
}

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
#[inline]
pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    parse_all(s, 0)
}

#[inline(never)]
fn parse_all(s: &str, depth: usize) -> Result<Color, ParseColorError> {
    if depth > MAX_DEPTH {
        return Err(ParseColorError::InvalidUnknown);
    }

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
        ) = (
            pp.value(), pp.space(),
            pp.value(), pp.space(),
        ) else {
            return Err(err);
        };

        let mut color_func = ColorFunc::Srgb;

        if err == ParseColorError::InvalidColor {
            if let Some(s) = pp.value() {
                if s.eq_ignore_ascii_case("srgb") {
                    // srgb
                } else if s.eq_ignore_ascii_case("srgb-linear") {
                    color_func = ColorFunc::SrgbLinear;
                } else {
                    return Err(err);
                }
            } else {
                return Err(err);
            };
            pp.space();
        }

        #[rustfmt::skip]
        let (
            Some(val1), true,
            Some(val2), true,
            Some(val3),
        ) = (
            pp.value(), pp.space(),
            pp.value(), pp.space(),
            pp.value(),
        ) else {
            return Err(err);
        };

        if !from.eq_ignore_ascii_case("from") {
            return Err(err);
        }

        let Ok(color) = parse_all(color, depth + 1) else {
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
            ParseColorError::InvalidColor => {
                match color_func {
                    ColorFunc::Srgb => {
                        // r, g, b, alpha [0..1]
                        let variables = [
                            ("r", color.r),
                            ("g", color.g),
                            ("b", color.b),
                            ("alpha", color.a),
                        ];
                        if let Some([r, g, b, a]) = parse_values(values, variables) {
                            return Ok(Color::new(r, g, b, a));
                        };
                    }
                    ColorFunc::SrgbLinear => {
                        // r, g, b, alpha [0..1]
                        let [r, g, b, a] = color.to_linear_rgba();
                        let variables = [("r", r), ("g", g), ("b", b), ("alpha", a)];
                        if let Some([r, g, b, a]) = parse_values(values, variables) {
                            return Ok(Color::from_linear_rgba(r, g, b, a));
                        };
                    } // TODO
                }
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
            s if s.eq_ignore_ascii_case("color") => ParseColorError::InvalidColor,
            _ => return Err(ParseColorError::InvalidFunction),
        };

        let s = &s[idx + 1..];

        if !s.is_ascii() {
            return Err(err);
        }

        let mut pp = ParamParser::new(s);
        pp.space();

        let mut color_func = ColorFunc::Srgb;

        if err == ParseColorError::InvalidColor {
            if let Some(s) = pp.value() {
                if s.eq_ignore_ascii_case("srgb") {
                    // srgb
                } else if s.eq_ignore_ascii_case("srgb-linear") {
                    color_func = ColorFunc::SrgbLinear;
                } else {
                    return Err(err);
                }
            } else {
                return Err(err);
            };
            pp.space();
        }

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
                if let (Some(h), Some((s, s_pct)), Some((l, l_pct))) = (
                    // hue
                    parse_angle(val0),
                    // saturation
                    parse_percent_or_float(val1),
                    // lightness
                    parse_percent_or_float(val2),
                ) {
                    // A bare number is on the 0..100 scale (CSS Color 4), while a
                    // percentage is already scaled to 0..1 by parse_percent_or_float.
                    let s = if s_pct { s } else { s / 100.0 };
                    let l = if l_pct { l } else { l / 100.0 };
                    return Ok(Color::from_hsla(h, s, l, alpha));
                }
            }
            ParseColorError::InvalidHwb => {
                if let (Some(h), Some((w, w_pct)), Some((b, b_pct))) = (
                    // hue
                    parse_angle(val0),
                    // whiteness
                    parse_percent_or_float(val1),
                    // blackness
                    parse_percent_or_float(val2),
                ) {
                    // A bare number is on the 0..100 scale (CSS Color 4), while a
                    // percentage is already scaled to 0..1 by parse_percent_or_float.
                    let w = if w_pct { w } else { w / 100.0 };
                    let b = if b_pct { b } else { b / 100.0 };
                    return Ok(Color::from_hwba(h, w, b, alpha));
                }
            }
            ParseColorError::InvalidHsv => {
                if let (Some(h), Some((s, s_pct)), Some((v, v_pct))) = (
                    // hue
                    parse_angle(val0),
                    // saturation
                    parse_percent_or_float(val1),
                    // value
                    parse_percent_or_float(val2),
                ) {
                    // A bare number is on the 0..100 scale, while a percentage is
                    // already scaled to 0..1 by parse_percent_or_float.
                    let s = if s_pct { s } else { s / 100.0 };
                    let v = if v_pct { v } else { v / 100.0 };
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
            ParseColorError::InvalidColor => {
                match color_func {
                    ColorFunc::Srgb => {
                        if let (Some((r, _)), Some((g, _)), Some((b, _))) = (
                            // red
                            parse_percent_or_float(val0),
                            // green
                            parse_percent_or_float(val1),
                            // blue
                            parse_percent_or_float(val2),
                        ) {
                            return Ok(Color { r, g, b, a: alpha });
                        }
                    }
                    ColorFunc::SrgbLinear => {
                        if let (Some((r, _)), Some((g, _)), Some((b, _))) = (
                            // red
                            parse_percent_or_float(val0),
                            // green
                            parse_percent_or_float(val1),
                            // blue
                            parse_percent_or_float(val2),
                        ) {
                            return Ok(Color::from_linear_rgba(r, g, b, alpha));
                        }
                    } // TODO
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
    if s.len() > 2 && s.len() < 21 {
        if let Some([r, g, b]) = NAMED_COLORS.get(s.into()) {
            return Ok(Color::from_rgba8(*r, *g, *b, 255));
        }
    }

    Err(ParseColorError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<Color, ParseColorError> {
    if s.bytes().any(|b| !b.is_ascii_hexdigit()) {
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

#[cfg(test)]
mod t {
    use super::*;

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
