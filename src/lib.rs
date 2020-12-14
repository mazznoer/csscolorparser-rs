//! # Overview
//!
//! Rust library for parsing CSS color string.
//!
//! ```rust
//! let c = csscolorparser::parse("rgb(100%,0%,0%)").unwrap();
//!
//! assert_eq!(c.rgba(), (1.0, 0.0, 0.0, 1.0));
//! assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(c.to_hex_string(), "#ff0000");
//! assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
//! ```

#![allow(clippy::many_single_char_names, clippy::float_cmp)]

use std::f64::consts::PI;

use std::error::Error as StdError;
use std::fmt;

/// The output color
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    /// Get red, green, blue, and alpha in the range [0.0, 1.0].
    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        (self.r, self.g, self.b, self.a)
    }

    /// Get red, green, blue, and alpha in the range [0, 255].
    pub fn rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }

    /// Get the hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        let (r, g, b, a) = self.rgba_u8();
        if a < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);
        }
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        let (r, g, b, _) = self.rgba_u8();
        if self.a < 1.0 {
            return format!("rgba({},{},{},{})", r, g, b, self.a);
        }
        format!("rgb({},{},{})", r, g, b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b, a) = self.rgba();
        write!(f, "Color({}, {}, {}, {})", r, g, b, a)
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidHex,
    InvalidRgb,
    InvalidHsl,
    InvalidHwb,
    InvalidHsv,
    InvalidUnknown,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidHex => f.write_str("Invalid hex format."),
            ParseError::InvalidRgb => f.write_str("Invalid rgb format."),
            ParseError::InvalidHsl => f.write_str("Invalid hsl format."),
            ParseError::InvalidHwb => f.write_str("Invalid hwb format."),
            ParseError::InvalidHsv => f.write_str("Invalid hsv format."),
            ParseError::InvalidUnknown => f.write_str("Invalid unknown format."),
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidHex => "Invalid hex format.",
            ParseError::InvalidRgb => "Invalid rgb format.",
            ParseError::InvalidHsl => "Invalid hsl format.",
            ParseError::InvalidHwb => "Invalid hwb format.",
            ParseError::InvalidHsv => "Invalid hsv format.",
            ParseError::InvalidUnknown => "Invalid unknown format.",
        }
    }
}

/// Parse CSS color string
///
/// # Examples
/// ```
/// // short hexadecimal format
/// let c = csscolorparser::parse("#ff0").unwrap();
/// assert_eq!(c.rgba(), (1., 1., 0., 1.));
/// assert_eq!(c.rgba_u8(), (255, 255, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ffff00");
/// assert_eq!(c.to_rgb_string(), "rgb(255,255,0)");
///
/// // hsl() format
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)").unwrap();
/// assert_eq!(c.rgba(), (1., 0., 0., 1.));
/// assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ff0000");
/// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
/// ```
///
/// ## Supported Format
///
/// It support named colors, hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.
///
/// ```text
/// ------ Example color format
/// transparent
/// gold
/// rebeccapurple
/// lime
/// #0f0
/// #0f0f
/// #00ff00
/// #00ff00ff
/// rgb(0,255,0)
/// rgb(0% 100% 0%)
/// rgb(0 255 0 / 100%)
/// rgba(0,255,0,1)
/// hsl(120,100%,50%)
/// hsl(120deg 100% 50%)
/// hsl(-240 100% 50%)
/// hsl(-240deg 100% 50%)
/// hsl(0.3333turn 100% 50%)
/// hsl(133.333grad 100% 50%)
/// hsl(2.0944rad 100% 50%)
/// hsla(120,100%,50%,100%)
/// hwb(120 0% 0%)
/// hwb(480deg 0% 0% / 100%)
/// hsv(120,100%,100%)
/// hsv(120deg 100% 100% / 100%)
/// ```
pub fn parse(s: &str) -> Result<Color, ParseError> {
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        });
    }

    if let Some(c) = named_colors(&s) {
        return Ok(Color {
            r: c[0] as f64 / 255.0,
            g: c[1] as f64 / 255.0,
            b: c[2] as f64 / 255.0,
            a: 1.0,
        });
    }

    if s.starts_with('#') {
        let s = s.strip_prefix("#").unwrap();
        if let Ok(c) = parse_hex(s) {
            return Ok(c);
        }
        return Err(ParseError::InvalidHex);
    }

    if let Some(op) = s.find('(') {
        if !s.ends_with(')') {
            return Err(ParseError::InvalidUnknown);
        }
        let fname = s.get(..op).unwrap().trim();
        let s = s.strip_suffix(")").unwrap();
        let s = s.get((op + 1)..).unwrap();

        let mut a = Some(1.0);
        let s = s.replace(",", " ");
        let s = s.replace("/", " ");
        let params: Vec<&str> = s.split_whitespace().collect();

        if fname == "rgb" || fname == "rgba" {
            if params.len() != 3 && params.len() != 4 {
                return Err(ParseError::InvalidRgb);
            }
            let r = parse_percent_or_255(params[0]);
            let g = parse_percent_or_255(params[1]);
            let b = parse_percent_or_255(params[2]);
            if params.len() == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if r.is_none() || g.is_none() || b.is_none() || a.is_none() {
                return Err(ParseError::InvalidRgb);
            }
            return Ok(Color {
                r: clamp0_1(r.unwrap()),
                g: clamp0_1(g.unwrap()),
                b: clamp0_1(b.unwrap()),
                a: clamp0_1(a.unwrap()),
            });
        } else if fname == "hsl" || fname == "hsla" {
            if params.len() != 3 && params.len() != 4 {
                return Err(ParseError::InvalidHsl);
            }
            let h = parse_angle(params[0]);
            let s = parse_percent_or_float(params[1]);
            let l = parse_percent_or_float(params[2]);
            if params.len() == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if h.is_none() || s.is_none() || l.is_none() || a.is_none() {
                return Err(ParseError::InvalidHsl);
            }
            let (r, g, b) = hsl_to_rgb(
                normalize_angle(h.unwrap()),
                clamp0_1(s.unwrap()),
                clamp0_1(l.unwrap()),
            );
            return Ok(Color {
                r: clamp0_1(r),
                g: clamp0_1(g),
                b: clamp0_1(b),
                a: clamp0_1(a.unwrap()),
            });
        } else if fname == "hwb" {
            if params.len() != 3 && params.len() != 4 {
                return Err(ParseError::InvalidHwb);
            }
            let h = parse_angle(params[0]);
            let w = parse_percent_or_float(params[1]);
            let b = parse_percent_or_float(params[2]);
            if params.len() == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if h.is_none() || w.is_none() || b.is_none() || a.is_none() {
                return Err(ParseError::InvalidHwb);
            }
            let (r, g, b) = hwb_to_rgb(
                normalize_angle(h.unwrap()),
                clamp0_1(w.unwrap()),
                clamp0_1(b.unwrap()),
            );
            return Ok(Color {
                r: clamp0_1(r),
                g: clamp0_1(g),
                b: clamp0_1(b),
                a: clamp0_1(a.unwrap()),
            });
        } else if fname == "hsv" {
            if params.len() != 3 && params.len() != 4 {
                return Err(ParseError::InvalidHsv);
            }
            let h = parse_angle(params[0]);
            let s = parse_percent_or_float(params[1]);
            let v = parse_percent_or_float(params[2]);
            if params.len() == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if h.is_none() || s.is_none() || v.is_none() || a.is_none() {
                return Err(ParseError::InvalidHsv);
            }
            let (r, g, b) = hsv_to_rgb(
                normalize_angle(h.unwrap()),
                clamp0_1(s.unwrap()),
                clamp0_1(v.unwrap()),
            );
            return Ok(Color {
                r: clamp0_1(r),
                g: clamp0_1(g),
                b: clamp0_1(b),
                a: clamp0_1(a.unwrap()),
            });
        }
    }

    Err(ParseError::InvalidUnknown)
}

fn parse_hex(s: &str) -> Result<Color, Box<dyn StdError>> {
    let n = s.len();
    let (r, g, b);
    let mut a = 255;
    if n == 3 || n == 4 {
        r = u8::from_str_radix(&s[0..1].repeat(2), 16)?;
        g = u8::from_str_radix(&s[1..2].repeat(2), 16)?;
        b = u8::from_str_radix(&s[2..3].repeat(2), 16)?;
        if n == 4 {
            a = u8::from_str_radix(&s[3..4].repeat(2), 16)?;
        }
    } else if n == 6 || n == 8 {
        r = u8::from_str_radix(&s[0..2], 16)?;
        g = u8::from_str_radix(&s[2..4], 16)?;
        b = u8::from_str_radix(&s[4..6], 16)?;
        if n == 8 {
            a = u8::from_str_radix(&s[6..8], 16)?;
        }
    } else {
        return Err(Box::new(ParseError::InvalidHex));
    }
    Ok(Color {
        r: r as f64 / 255.,
        g: g as f64 / 255.,
        b: b as f64 / 255.,
        a: a as f64 / 255.,
    })
}

fn hue_to_rgb(n1: f64, n2: f64, h: f64) -> f64 {
    let h = h % 6.0;
    if h < 1.0 {
        return n1 + ((n2 - n1) * h);
    }
    if h < 3.0 {
        return n2;
    }
    if h < 4.0 {
        return n1 + ((n2 - n1) * (4.0 - h));
    }
    n1
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0.0 {
        return (l, l, l);
    }
    let n2;
    if l < 0.5 {
        n2 = l * (1.0 + s);
    } else {
        n2 = l + s - (l * s);
    }
    let n1 = 2.0 * l - n2;
    let h = h / 60.0;
    let r = hue_to_rgb(n1, n2, h + 2.0);
    let g = hue_to_rgb(n1, n2, h);
    let b = hue_to_rgb(n1, n2, h - 2.0);
    (r, g, b)
}

fn hwb_to_rgb(hue: f64, white: f64, black: f64) -> (f64, f64, f64) {
    if white + black >= 1.0 {
        let l = white / (white + black);
        return (l, l, l);
    }
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.5);
    let r = r * (1.0 - white - black) + white;
    let g = g * (1.0 - white - black) + white;
    let b = b * (1.0 - white - black) + white;
    (r, g, b)
}

fn hsv_to_hsl(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let mut s = s;
    let l = (2.0 - s) * v / 2.0;
    if l != 0.0 {
        if l == 1.0 {
            s = 0.0;
        } else if l < 0.5 {
            s = s * v / (l * 2.0);
        } else {
            s = s * v / (2.0 - l * 2.0);
        }
    }
    (h, s, l)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let (h, s, l) = hsv_to_hsl(h, s, v);
    hsl_to_rgb(h, s, l)
}

fn parse_percent_or_float(s: &str) -> Option<f64> {
    if s.ends_with('%') {
        let s = s.strip_suffix("%").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t / 100.);
        }
        return None;
    }
    if let Ok(t) = s.parse::<f64>() {
        return Some(t);
    }
    None
}

fn parse_percent_or_255(s: &str) -> Option<f64> {
    if s.ends_with('%') {
        let s = s.strip_suffix("%").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t / 100.);
        }
        return None;
    }
    if let Ok(t) = s.parse::<f64>() {
        return Some(t / 255.);
    }
    None
}

fn parse_angle(s: &str) -> Option<f64> {
    if s.ends_with("deg") {
        let s = s.strip_suffix("deg").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t);
        }
        return None;
    }
    if s.ends_with("grad") {
        let s = s.strip_suffix("grad").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 360. / 400.);
        }
        return None;
    }
    if s.ends_with("rad") {
        let s = s.strip_suffix("rad").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 180. / PI);
        }
        return None;
    }
    if s.ends_with("turn") {
        let s = s.strip_suffix("turn").unwrap();
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 360.);
        }
        return None;
    }
    if let Ok(t) = s.parse::<f64>() {
        return Some(t);
    }
    None
}

fn normalize_angle(t: f64) -> f64 {
    let mut t = t % 360.0;
    if t < 0.0 {
        t += 360.0;
    }
    t
}

fn clamp0_1(t: f64) -> f64 {
    if t < 0.0 {
        return 0.0;
    }
    if t > 1.0 {
        return 1.0;
    }
    t
}

// https://www.w3.org/TR/css-color-4/#named-colors

fn named_colors(s: &str) -> Option<[u8; 3]> {
    match s {
        "aliceblue" => Some([240, 248, 255]),
        "antiquewhite" => Some([250, 235, 215]),
        "aqua" => Some([0, 255, 255]),
        "aquamarine" => Some([127, 255, 212]),
        "azure" => Some([240, 255, 255]),
        "beige" => Some([245, 245, 220]),
        "bisque" => Some([255, 228, 196]),
        "black" => Some([0, 0, 0]),
        "blanchedalmond" => Some([255, 235, 205]),
        "blue" => Some([0, 0, 255]),
        "blueviolet" => Some([138, 43, 226]),
        "brown" => Some([165, 42, 42]),
        "burlywood" => Some([222, 184, 135]),
        "cadetblue" => Some([95, 158, 160]),
        "chartreuse" => Some([127, 255, 0]),
        "chocolate" => Some([210, 105, 30]),
        "coral" => Some([255, 127, 80]),
        "cornflowerblue" => Some([100, 149, 237]),
        "cornsilk" => Some([255, 248, 220]),
        "crimson" => Some([220, 20, 60]),
        "cyan" => Some([0, 255, 255]),
        "darkblue" => Some([0, 0, 139]),
        "darkcyan" => Some([0, 139, 139]),
        "darkgoldenrod" => Some([184, 134, 11]),
        "darkgray" => Some([169, 169, 169]),
        "darkgreen" => Some([0, 100, 0]),
        "darkgrey" => Some([169, 169, 169]),
        "darkkhaki" => Some([189, 183, 107]),
        "darkmagenta" => Some([139, 0, 139]),
        "darkolivegreen" => Some([85, 107, 47]),
        "darkorange" => Some([255, 140, 0]),
        "darkorchid" => Some([153, 50, 204]),
        "darkred" => Some([139, 0, 0]),
        "darksalmon" => Some([233, 150, 122]),
        "darkseagreen" => Some([143, 188, 143]),
        "darkslateblue" => Some([72, 61, 139]),
        "darkslategray" => Some([47, 79, 79]),
        "darkslategrey" => Some([47, 79, 79]),
        "darkturquoise" => Some([0, 206, 209]),
        "darkviolet" => Some([148, 0, 211]),
        "deeppink" => Some([255, 20, 147]),
        "deepskyblue" => Some([0, 191, 255]),
        "dimgray" => Some([105, 105, 105]),
        "dimgrey" => Some([105, 105, 105]),
        "dodgerblue" => Some([30, 144, 255]),
        "firebrick" => Some([178, 34, 34]),
        "floralwhite" => Some([255, 250, 240]),
        "forestgreen" => Some([34, 139, 34]),
        "fuchsia" => Some([255, 0, 255]),
        "gainsboro" => Some([220, 220, 220]),
        "ghostwhite" => Some([248, 248, 255]),
        "gold" => Some([255, 215, 0]),
        "goldenrod" => Some([218, 165, 32]),
        "gray" => Some([128, 128, 128]),
        "green" => Some([0, 128, 0]),
        "greenyellow" => Some([173, 255, 47]),
        "grey" => Some([128, 128, 128]),
        "honeydew" => Some([240, 255, 240]),
        "hotpink" => Some([255, 105, 180]),
        "indianred" => Some([205, 92, 92]),
        "indigo" => Some([75, 0, 130]),
        "ivory" => Some([255, 255, 240]),
        "khaki" => Some([240, 230, 140]),
        "lavender" => Some([230, 230, 250]),
        "lavenderblush" => Some([255, 240, 245]),
        "lawngreen" => Some([124, 252, 0]),
        "lemonchiffon" => Some([255, 250, 205]),
        "lightblue" => Some([173, 216, 230]),
        "lightcoral" => Some([240, 128, 128]),
        "lightcyan" => Some([224, 255, 255]),
        "lightgoldenrodyellow" => Some([250, 250, 210]),
        "lightgray" => Some([211, 211, 211]),
        "lightgreen" => Some([144, 238, 144]),
        "lightgrey" => Some([211, 211, 211]),
        "lightpink" => Some([255, 182, 193]),
        "lightsalmon" => Some([255, 160, 122]),
        "lightseagreen" => Some([32, 178, 170]),
        "lightskyblue" => Some([135, 206, 250]),
        "lightslategray" => Some([119, 136, 153]),
        "lightslategrey" => Some([119, 136, 153]),
        "lightsteelblue" => Some([176, 196, 222]),
        "lightyellow" => Some([255, 255, 224]),
        "lime" => Some([0, 255, 0]),
        "limegreen" => Some([50, 205, 50]),
        "linen" => Some([250, 240, 230]),
        "magenta" => Some([255, 0, 255]),
        "maroon" => Some([128, 0, 0]),
        "mediumaquamarine" => Some([102, 205, 170]),
        "mediumblue" => Some([0, 0, 205]),
        "mediumorchid" => Some([186, 85, 211]),
        "mediumpurple" => Some([147, 112, 219]),
        "mediumseagreen" => Some([60, 179, 113]),
        "mediumslateblue" => Some([123, 104, 238]),
        "mediumspringgreen" => Some([0, 250, 154]),
        "mediumturquoise" => Some([72, 209, 204]),
        "mediumvioletred" => Some([199, 21, 133]),
        "midnightblue" => Some([25, 25, 112]),
        "mintcream" => Some([245, 255, 250]),
        "mistyrose" => Some([255, 228, 225]),
        "moccasin" => Some([255, 228, 181]),
        "navajowhite" => Some([255, 222, 173]),
        "navy" => Some([0, 0, 128]),
        "oldlace" => Some([253, 245, 230]),
        "olive" => Some([128, 128, 0]),
        "olivedrab" => Some([107, 142, 35]),
        "orange" => Some([255, 165, 0]),
        "orangered" => Some([255, 69, 0]),
        "orchid" => Some([218, 112, 214]),
        "palegoldenrod" => Some([238, 232, 170]),
        "palegreen" => Some([152, 251, 152]),
        "paleturquoise" => Some([175, 238, 238]),
        "palevioletred" => Some([219, 112, 147]),
        "papayawhip" => Some([255, 239, 213]),
        "peachpuff" => Some([255, 218, 185]),
        "peru" => Some([205, 133, 63]),
        "pink" => Some([255, 192, 203]),
        "plum" => Some([221, 160, 221]),
        "powderblue" => Some([176, 224, 230]),
        "purple" => Some([128, 0, 128]),
        "rebeccapurple" => Some([102, 51, 153]),
        "red" => Some([255, 0, 0]),
        "rosybrown" => Some([188, 143, 143]),
        "royalblue" => Some([65, 105, 225]),
        "saddlebrown" => Some([139, 69, 19]),
        "salmon" => Some([250, 128, 114]),
        "sandybrown" => Some([244, 164, 96]),
        "seagreen" => Some([46, 139, 87]),
        "seashell" => Some([255, 245, 238]),
        "sienna" => Some([160, 82, 45]),
        "silver" => Some([192, 192, 192]),
        "skyblue" => Some([135, 206, 235]),
        "slateblue" => Some([106, 90, 205]),
        "slategray" => Some([112, 128, 144]),
        "slategrey" => Some([112, 128, 144]),
        "snow" => Some([255, 250, 250]),
        "springgreen" => Some([0, 255, 127]),
        "steelblue" => Some([70, 130, 180]),
        "tan" => Some([210, 180, 140]),
        "teal" => Some([0, 128, 128]),
        "thistle" => Some([216, 191, 216]),
        "tomato" => Some([255, 99, 71]),
        "turquoise" => Some([64, 224, 208]),
        "violet" => Some([238, 130, 238]),
        "wheat" => Some([245, 222, 179]),
        "white" => Some([255, 255, 255]),
        "whitesmoke" => Some([245, 245, 245]),
        "yellow" => Some([255, 255, 0]),
        "yellowgreen" => Some([154, 205, 50]),
        _ => None,
    }
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

    #[test]
    fn test_normalize_angle() {
        let data = vec![
            (0.0, 0.0),
            (360.0, 0.0),
            (400.0, 40.0),
            (1155.0, 75.0),
            (-360.0, 0.0),
            (-90.0, 270.0),
            (-765.0, 315.0),
        ];
        for (x, expected) in data {
            let c = normalize_angle(x);
            assert_eq!(expected, c);
        }
    }
}
