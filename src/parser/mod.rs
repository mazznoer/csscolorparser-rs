use std::{error, fmt};

use crate::Color;

#[cfg(feature = "named-colors")]
mod named_colors;

#[cfg(feature = "named-colors")]
use named_colors::NAMED_COLORS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseColorError {
    InvalidHex,
    InvalidRgb,
    InvalidHsl,
    InvalidHwb,
    InvalidHsv,
    #[cfg(feature = "lab")]
    InvalidLab,
    #[cfg(feature = "lab")]
    InvalidLch,
    InvalidFunction,
    InvalidUnknown,
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseColorError::InvalidHex => f.write_str("invalid hex format"),
            ParseColorError::InvalidRgb => f.write_str("invalid rgb format"),
            ParseColorError::InvalidHsl => f.write_str("invalid hsl format"),
            ParseColorError::InvalidHwb => f.write_str("invalid hwb format"),
            ParseColorError::InvalidHsv => f.write_str("invalid hsv format"),
            #[cfg(feature = "lab")]
            ParseColorError::InvalidLab => f.write_str("invalid lab format"),
            #[cfg(feature = "lab")]
            ParseColorError::InvalidLch => f.write_str("invalid lch format"),
            ParseColorError::InvalidFunction => f.write_str("invalid color function"),
            ParseColorError::InvalidUnknown => f.write_str("invalid unknown format"),
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
/// assert_eq!(c.rgba(), (1.0, 1.0, 0.0, 1.0));
/// assert_eq!(c.rgba_u8(), (255, 255, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ffff00");
/// assert_eq!(c.to_rgb_string(), "rgb(255,255,0)");
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)")?;
///
/// assert_eq!(c.rgba(), (1.0, 0.0, 0.0, 1.0));
/// assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ff0000");
/// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
/// # Ok(())
/// # }
/// ```
pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(Color::from_rgba(0.0, 0.0, 0.0, 0.0));
    }

    // Named colors
    #[cfg(feature = "named-colors")]
    if let Some([r, g, b]) = NAMED_COLORS.get(&*s) {
        return Ok(Color::from_rgb_u8(*r, *g, *b));
    }

    // Hex format
    if let Some(s) = s.strip_prefix('#') {
        if let Ok(c) = parse_hex(s) {
            return Ok(c);
        }
        return Err(ParseColorError::InvalidHex);
    }

    if let (Some(i), Some(s)) = (s.find('('), s.strip_suffix(')')) {
        let fname = &s[..i].trim_end();
        let s = &s[i + 1..].replace(',', " ").replace('/', " ");
        let params = s.split_whitespace().collect::<Vec<&str>>();
        let p_len = params.len();

        match *fname {
            "rgb" | "rgba" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidRgb);
                }

                let r = parse_percent_or_255(params[0]);
                let g = parse_percent_or_255(params[1]);
                let b = parse_percent_or_255(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(r), Some(g), Some(b), Some(a)) = (r, g, b, a) {
                    return Ok(Color {
                        r: r.clamp(0.0, 1.0),
                        g: g.clamp(0.0, 1.0),
                        b: b.clamp(0.0, 1.0),
                        a: a.clamp(0.0, 1.0),
                    });
                }

                return Err(ParseColorError::InvalidRgb);
            }
            "hsl" | "hsla" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidHsl);
                }

                let h = parse_angle(params[0]);
                let s = parse_percent_or_float(params[1]);
                let l = parse_percent_or_float(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(h), Some(s), Some(l), Some(a)) = (h, s, l, a) {
                    return Ok(Color::from_hsla(h, s, l, a));
                }

                return Err(ParseColorError::InvalidHsl);
            }
            "hwb" | "hwba" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidHwb);
                }

                let h = parse_angle(params[0]);
                let w = parse_percent_or_float(params[1]);
                let b = parse_percent_or_float(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(h), Some(w), Some(b), Some(a)) = (h, w, b, a) {
                    return Ok(Color::from_hwba(h, w, b, a));
                }

                return Err(ParseColorError::InvalidHwb);
            }
            "hsv" | "hsva" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidHsv);
                }

                let h = parse_angle(params[0]);
                let s = parse_percent_or_float(params[1]);
                let v = parse_percent_or_float(params[2]);

                let a = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(h), Some(s), Some(v), Some(a)) = (h, s, v, a) {
                    return Ok(Color::from_hsva(h, s, v, a));
                }

                return Err(ParseColorError::InvalidHsv);
            }
            #[cfg(feature = "lab")]
            "lab" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidLab);
                }

                let l = parse_percent_or_float(params[0]);
                let a = parse_percent_or_float(params[1]);
                let b = parse_percent_or_float(params[2]);

                let alpha = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(l), Some(a), Some(b), Some(alpha)) = (l, a, b, alpha) {
                    return Ok(Color::from_lab(l.max(0.0) * 100.0, a, b, alpha));
                }

                return Err(ParseColorError::InvalidLab);
            }
            #[cfg(feature = "lab")]
            "lch" => {
                if p_len != 3 && p_len != 4 {
                    return Err(ParseColorError::InvalidLch);
                }

                let l = parse_percent_or_float(params[0]);
                let c = parse_percent_or_float(params[1]);
                let h = parse_angle(params[2]);

                let alpha = if p_len == 4 {
                    parse_percent_or_float(params[3])
                } else {
                    Some(1.0)
                };

                if let (Some(l), Some(c), Some(h), Some(alpha)) = (l, c, h, alpha) {
                    return Ok(Color::from_lch(
                        l.max(0.0) * 100.0,
                        c.max(0.0),
                        h.to_radians(),
                        alpha,
                    ));
                }

                return Err(ParseColorError::InvalidLch);
            }
            _ => {
                return Err(ParseColorError::InvalidFunction);
            }
        }
    }

    // Hex format without prefix '#'
    if let Ok(c) = parse_hex(&s) {
        return Ok(c);
    }

    Err(ParseColorError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<Color, Box<dyn error::Error>> {
    if !s.is_ascii() {
        return Err(Box::new(ParseColorError::InvalidHex));
    }

    let n = s.len();

    let (r, g, b, a) = if n == 3 || n == 4 {
        let r = u8::from_str_radix(&s[0..1].repeat(2), 16)?;
        let g = u8::from_str_radix(&s[1..2].repeat(2), 16)?;
        let b = u8::from_str_radix(&s[2..3].repeat(2), 16)?;

        let a = if n == 4 {
            u8::from_str_radix(&s[3..4].repeat(2), 16)?
        } else {
            255
        };

        (r, g, b, a)
    } else if n == 6 || n == 8 {
        let r = u8::from_str_radix(&s[0..2], 16)?;
        let g = u8::from_str_radix(&s[2..4], 16)?;
        let b = u8::from_str_radix(&s[4..6], 16)?;

        let a = if n == 8 {
            u8::from_str_radix(&s[6..8], 16)?
        } else {
            255
        };

        (r, g, b, a)
    } else {
        return Err(Box::new(ParseColorError::InvalidHex));
    };

    Ok(Color::from_rgba_u8(r, g, b, a))
}

fn parse_percent_or_float(s: &str) -> Option<f64> {
    if let Some(s) = s.strip_suffix('%') {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t / 100.0);
        }
        return None;
    }

    if let Ok(t) = s.parse::<f64>() {
        return Some(t);
    }

    None
}

fn parse_percent_or_255(s: &str) -> Option<f64> {
    if let Some(s) = s.strip_suffix('%') {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t / 100.0);
        }
        return None;
    }

    if let Ok(t) = s.parse::<f64>() {
        return Some(t / 255.0);
    }

    None
}

fn parse_angle(s: &str) -> Option<f64> {
    if let Some(s) = s.strip_suffix("deg") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t);
        }
        return None;
    }

    if let Some(s) = s.strip_suffix("grad") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 360.0 / 400.0);
        }
        return None;
    }

    if let Some(s) = s.strip_suffix("rad") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t.to_degrees());
        }
        return None;
    }

    if let Some(s) = s.strip_suffix("turn") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 360.0);
        }
        return None;
    }

    if let Ok(t) = s.parse::<f64>() {
        return Some(t);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_angle() {
        let data = vec![
            ("360", 360.0),
            ("127.356", 127.356),
            ("+120deg", 120.0),
            ("90deg", 90.0),
            ("-127deg", -127.0),
            ("100grad", 90.0),
            ("1.5707963267948966rad", 90.0),
            ("0.25turn", 90.0),
            ("-0.25turn", -90.0),
        ];
        for (s, expected) in data {
            let c = parse_angle(s);
            assert_eq!(Some(expected), c);
        }
    }
}
