use crate::utils::remap;
use crate::{Color, ParseColorError};

#[cfg(feature = "named-colors")]
use crate::NAMED_COLORS;

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
#[allow(clippy::needless_match)]
pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    let s = s.trim();

    let err = match parse_abs(s) {
        Ok(c) => return Ok(c),
        Err(e @ ParseColorError::InvalidHex) => return Err(e),
        Err(e @ ParseColorError::InvalidUnknown) => return Err(e),
        e => e,
    };

    if !s.is_ascii() {
        return err;
    }

    if let (Some(idx), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..idx];
        let mut params = split_by_space(&s[idx + 1..]);

        if let Some(s) = params.next() {
            if !s.eq_ignore_ascii_case("from") {
                return err;
            }
        } else {
            return err;
        };

        // parse next value as color
        let color = if let Some(s) = params.next() {
            if let Ok(color) = parse(s) {
                color
            } else {
                return err;
            }
        } else {
            return err;
        };

        let (Some(val1), Some(val2), Some(val3)) = (params.next(), params.next(), params.next())
        else {
            return err;
        };

        let val4 = if let (Some("/"), Some(alpha)) = (params.next(), params.next()) {
            alpha
        } else {
            "alpha"
        };

        if fname.eq_ignore_ascii_case("rgb") {
            // r, g, b [0..255]
            // alpha   [0..1]
            let variables = [
                ("r", color.r * 255.0),
                ("g", color.g * 255.0),
                ("b", color.b * 255.0),
                ("alpha", color.a),
            ];
            if let (Some(r), Some(g), Some(b), Some(a)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::new(r / 255.0, g / 255.0, b / 255.0, a));
            };
            return err;
        } else if fname.eq_ignore_ascii_case("hwb") {
            // h    [0..360]
            // w, b [0..100]
            let [h, w, b, a] = color.to_hwba();
            let variables = [("h", h), ("w", w * 100.0), ("b", b * 100.0), ("alpha", a)];
            if let (Some(h), Some(w), Some(b), Some(a)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::from_hwba(h, w / 100.0, b / 100.0, a));
            };
            return err;
        } else if fname.eq_ignore_ascii_case("hsl") {
            // h    [0..360]
            // s, l [0..100]
            let [h, s, l, a] = color.to_hsla();
            let variables = [("h", h), ("s", s * 100.0), ("l", l * 100.0), ("alpha", a)];
            if let (Some(h), Some(s), Some(l), Some(a)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::from_hsla(
                    h,
                    (s / 100.0).clamp(0.0, 1.0),
                    (l / 100.0).clamp(0.0, 1.0),
                    a,
                ));
            };
            return err;
        } else if fname.eq_ignore_ascii_case("hsv") {
            // h    [0..360]
            // s, v [0..100]
            let [h, s, v, a] = color.to_hsva();
            let variables = [("h", h), ("s", s * 100.0), ("v", v * 100.0), ("alpha", a)];
            if let (Some(h), Some(s), Some(v), Some(a)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::from_hsva(h, s / 100.0, v / 100.0, a));
            };
            return err;
        } else if fname.eq_ignore_ascii_case("lab") {
            #[cfg(feature = "lab")]
            {
                // l    [0..100]
                // a, b [-125..125]
                let [l, a, b, alpha] = color.to_laba();
                let variables = [("l", l), ("a", a), ("b", b), ("alpha", alpha)];
                if let (Some(l), Some(a), Some(b), Some(alpha)) = (
                    parse_value(val1, variables),
                    parse_value(val2, variables),
                    parse_value(val3, variables),
                    parse_value(val4, variables),
                ) {
                    return Ok(Color::from_laba(l.max(0.0), a, b, alpha));
                };
            }
            return err;
        } else if fname.eq_ignore_ascii_case("lch") {
            #[cfg(feature = "lab")]
            {
                // l [0..100]
                // c [0..150]
                // h [0..360]
                let [l, c, h, a] = color.to_lcha();
                let variables = [("l", l), ("c", c), ("h", h.to_degrees()), ("alpha", a)];
                if let (Some(l), Some(c), Some(h), Some(a)) = (
                    parse_value(val1, variables),
                    parse_value(val2, variables),
                    parse_value(val3, variables),
                    parse_value(val4, variables),
                ) {
                    return Ok(Color::from_lcha(l.max(0.0), c.max(0.0), h.to_radians(), a));
                };
            }
            return err;
        } else if fname.eq_ignore_ascii_case("oklab") {
            // l    [0..1]
            // a, b [-0.4 .. 0.4]
            let [l, a, b, alpha] = color.to_oklaba();
            let variables = [("l", l), ("a", a), ("b", b), ("alpha", alpha)];
            if let (Some(l), Some(a), Some(b), Some(alpha)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::from_oklaba(l.max(0.0), a, b, alpha));
            };
            return err;
        } else if fname.eq_ignore_ascii_case("oklch") {
            // l [0..1]
            // c [0..0.4]
            // h [0..360]
            let [l, c, h, a] = to_oklcha(&color);
            let variables = [("l", l), ("c", c), ("h", h.to_degrees()), ("alpha", a)];
            if let (Some(l), Some(c), Some(h), Some(a)) = (
                parse_value(val1, variables),
                parse_value(val2, variables),
                parse_value(val3, variables),
                parse_value(val4, variables),
            ) {
                return Ok(Color::from_oklcha(
                    l.max(0.0),
                    c.max(0.0),
                    h.to_radians(),
                    a,
                ));
            };
            return err;
        }
    }

    err
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

fn parse_value(s: &str, variables: [(&str, f32); 4]) -> Option<f32> {
    let parse_v = |s: &str| {
        if let Ok(value) = s.parse() {
            return Some(value);
        };
        for (var, value) in variables {
            if s.eq_ignore_ascii_case(var) {
                return Some(value);
            }
        }
        None
    };

    let s = s.trim();

    if let Some(t) = parse_v(s) {
        return Some(t);
    }

    if let Some(s) = strip_prefix(s, "calc(") {
        if let Some(s) = s.strip_suffix(')') {
            let mut it = s.split_ascii_whitespace();

            let (Some(val1), Some(op), Some(val2)) = (it.next(), it.next(), it.next()) else {
                return None;
            };

            if it.next().is_some() {
                return None;
            }

            let (Some(val1), Some(val2)) = (parse_v(val1), parse_v(val2)) else {
                return None;
            };

            match op {
                "+" => return Some(val1 + val2),
                "-" => return Some(val1 - val2),
                "*" => return Some(val1 * val2),
                "/" => {
                    if val2 == 0.0 {
                        return None;
                    }
                    return Some(val1 / val2);
                }
                _ => return None,
            }
        }
    }

    None
}

fn to_oklcha(color: &Color) -> [f32; 4] {
    let [l, a, b, alpha] = color.to_oklaba();
    let c = (a * a + b * b).sqrt();
    let h = b.atan2(a);
    [l, c, h, alpha]
}

struct SplitBySpace<'a> {
    s: &'a str,
    pos: usize,
    inside: usize,
}

impl<'a> Iterator for SplitBySpace<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.s.len() {
            return None;
        }

        let start = self.pos;
        let mut found_value = false;

        for (i, c) in self.s[self.pos..].chars().enumerate() {
            if c.is_whitespace() && self.inside == 0 {
                if found_value {
                    let end = self.pos + i;
                    self.pos = end + 1;
                    return Some(&self.s[start..end]);
                }
                self.pos += 1;
                return self.next();
            } else if c == '(' {
                self.inside += 1;
                found_value = true;
            } else if c == ')' {
                if self.inside > 0 {
                    self.inside -= 1;
                }
                found_value = true;
            } else if !c.is_whitespace() {
                found_value = true;
            }
        }

        if found_value {
            self.pos = self.s.len();
            Some(&self.s[start..])
        } else {
            None
        }
    }
}

fn split_by_space(s: &str) -> SplitBySpace {
    SplitBySpace {
        s,
        pos: 0,
        inside: 0,
    }
}

// strip prefix ignore case
fn strip_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
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
