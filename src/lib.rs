//! [![github](https://img.shields.io/static/v1?logo=github&label=github&message=mazznoer/csscolorparser-rs&color=8da0cb)](https://github.com/mazznoer/csscolorparser-rs/)
//! [![crates.io](https://img.shields.io/crates/v/csscolorparser.svg)](https://crates.io/crates/csscolorparser)
//!
//! # Overview
//!
//! Rust library to parse CSS color string as defined in the W3C's [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/).
//!
//! ## Supported Color Format
//!
//! * [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
//! * RGB hexadecimal
//!      + Short format `#rgb`
//!      + Short format with alpha `#rgba`
//!      + Long format `#rrggbb`
//!      + Long format with alpha `#rrggbbaa`
//! * `rgb()` and `rgba()`
//! * `hsl()` and `hsla()`
//! * `hwb()`
//! * `hsv()` - Not in CSS standard.
//!
//! Not yet supported: `lab()`, `lch()`.
//!
//! ### Example Color Format
//!
//! ```text
//! transparent
//! gold
//! rebeccapurple
//! lime
//! #0f0
//! #0f0f
//! #00ff00
//! #00ff00ff
//! rgb(0,255,0)
//! rgb(0% 100% 0%)
//! rgb(0 255 0 / 100%)
//! rgba(0,255,0,1)
//! hsl(120,100%,50%)
//! hsl(120deg 100% 50%)
//! hsl(-240 100% 50%)
//! hsl(-240deg 100% 50%)
//! hsl(0.3333turn 100% 50%)
//! hsl(133.333grad 100% 50%)
//! hsl(2.0944rad 100% 50%)
//! hsla(120,100%,50%,100%)
//! hwb(120 0% 0%)
//! hwb(480deg 0% 0% / 100%)
//! hsv(120,100%,100%)
//! hsv(120deg 100% 100% / 100%)
//! ```
//!
//! ## Usage
//!
//! Add `csscolorparser` to your `Cargo.toml`
//!
//! ```text
//! [dependencies]
//! csscolorparser = "0.2.0"
//! ```
//!
//! ## Examples
//!
//! Using [`csscolorparser::parse()`](fn.parse.html) function.
//!
//! ```rust
//! let c = csscolorparser::parse("rgb(100%,0%,0%)").unwrap();
//!
//! assert_eq!(c.rgba(), (1.0, 0.0, 0.0, 1.0));
//! assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(c.to_hex_string(), "#ff0000");
//! assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
//! ```
//!
//! Using `parse()` method on string.
//!
//! ```rust
//! use csscolorparser::Color;
//!
//! let c = "#ff00007f".parse::<Color>().unwrap();
//!
//! assert_eq!(c.rgba_u8(), (255, 0, 0, 127));
//! assert_eq!(c.to_hex_string(), "#ff00007f");
//! ```
//!
//! Using [`Color::from_html()`](struct.Color.html#method.from_html).
//!
//! ```rust
//! use csscolorparser::Color;
//!
//! let c = Color::from_html("skyblue").unwrap();
//!
//! assert_eq!(c.rgba_u8(), (135, 206, 235, 255));
//! assert_eq!(c.to_hex_string(), "#87ceeb");
//! assert_eq!(c.to_rgb_string(), "rgb(135,206,235)");
//! ```

#![allow(clippy::many_single_char_names)]

use phf::phf_map;

use std::error::Error as StdError;
use std::f64::consts::PI;
use std::fmt;
use std::str::FromStr;

/// The color
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, a: 1. }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            a: 1.,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            a: a as f64 / 255.,
        }
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Color {
        Color::from_hsva(h, s, v, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsva(h: f64, s: f64, v: f64, a: f64) -> Color {
        let (r, g, b) = hsv_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(v));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Color {
        Color::from_hsla(h, s, l, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsla(h: f64, s: f64, l: f64, a: f64) -> Color {
        let (r, g, b) = hsl_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(l));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    pub fn from_hwb(h: f64, w: f64, b: f64) -> Color {
        Color::from_hwba(h, w, b, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hwba(h: f64, w: f64, b: f64, a: f64) -> Color {
        let (r, g, b) = hwb_to_rgb(normalize_angle(h), clamp0_1(w), clamp0_1(b));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), a)
    }

    /// Create color from CSS color string.
    ///
    /// # Examples
    /// ```
    /// use csscolorparser::Color;
    ///
    /// let c = Color::from_html("red").unwrap();
    ///
    /// assert_eq!(c.rgba(), (1., 0., 0., 1.));
    /// assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    /// assert_eq!(c.to_hex_string(), "#ff0000");
    /// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    /// ```
    pub fn from_html(s: &str) -> Result<Color, ParseError> {
        parse(s)
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        (self.r, self.g, self.b, self.a)
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.).round() as u8,
            (self.g * 255.).round() as u8,
            (self.b * 255.).round() as u8,
            (self.a * 255.).round() as u8,
        )
    }

    /// Get the red value [0..1].
    pub fn red(&self) -> f64 {
        self.r
    }

    /// Get the green value [0..1].
    pub fn green(&self) -> f64 {
        self.g
    }

    /// Get the blue value [0..1].
    pub fn blue(&self) -> f64 {
        self.b
    }

    /// Get the alpha value [0..1].
    pub fn alpha(&self) -> f64 {
        self.a
    }

    /// Returns: `(h, s, v, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsva(&self) -> (f64, f64, f64, f64) {
        let (h, s, v) = rgb_to_hsv(self.r, self.g, self.b);
        (h, s, v, self.a)
    }

    /// Returns: `(h, s, l, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> (f64, f64, f64, f64) {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        (h, s, l, self.a)
    }

    /// Returns: `(h, w, b, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hwba(&self) -> (f64, f64, f64, f64) {
        let (h, w, b) = rgb_to_hwb(self.r, self.g, self.b);
        (h, w, b, self.a)
    }

    /// Get the RGB hexadecimal color string.
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
        if self.a < 1. {
            return format!("rgba({},{},{},{})", r, g, b, self.a);
        }
        format!("rgb({},{},{})", r, g, b)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Color, t: f64) -> Color {
        Color {
            r: self.r + t * (other.r - self.r),
            g: self.g + t * (other.g - self.g),
            b: self.b + t * (other.b - self.b),
            a: self.a + t * (other.a - self.a),
        }
    }

    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_lrgb(&self, other: &Color, t: f64) -> Color {
        Color {
            r: (self.r.powi(2) * (1. - t) + other.r.powi(2) * t).sqrt(),
            g: (self.g.powi(2) * (1. - t) + other.g.powi(2) * t).sqrt(),
            b: (self.b.powi(2) * (1. - t) + other.b.powi(2) * t).sqrt(),
            a: (self.a.powi(2) * (1. - t) + other.a.powi(2) * t).sqrt(),
        }
    }

    /// Blend this color with the other one, in the HSV color-space. `t` in the range [0..1].
    pub fn interpolate_hsv(&self, other: &Color, t: f64) -> Color {
        let (h1, s1, v1, a1) = self.to_hsva();
        let (h2, s2, v2, a2) = other.to_hsva();
        Color::from_hsva(
            interp_angle(h1, h2, t),
            s1 + t * (s2 - s1),
            v1 + t * (v2 - v1),
            a1 + t * (a2 - a1),
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b, a) = self.rgba();
        write!(f, "RGBA({},{},{},{})", r, g, b, a)
    }
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
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

impl StdError for ParseError {}

/// Parse CSS color string
///
/// # Examples
///
/// ```
/// let c = csscolorparser::parse("#ff0").unwrap();
/// assert_eq!(c.rgba(), (1., 1., 0., 1.));
/// assert_eq!(c.rgba_u8(), (255, 255, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ffff00");
/// assert_eq!(c.to_rgb_string(), "rgb(255,255,0)");
/// ```
///
/// ```
/// let c = csscolorparser::parse("hsl(360deg,100%,50%)").unwrap();
/// assert_eq!(c.rgba(), (1., 0., 0., 1.));
/// assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(c.to_hex_string(), "#ff0000");
/// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
/// ```
pub fn parse(s: &str) -> Result<Color, ParseError> {
    let s = s.trim().to_lowercase();

    if s == "transparent" {
        return Ok(Color::from_rgba(0., 0., 0., 0.));
    }

    if let Some(c) = NAMED_COLORS.get(&*s) {
        return Ok(Color::from_rgb_u8(c[0], c[1], c[2]));
    }

    if let Some(s) = s.strip_prefix("#") {
        if let Ok(c) = parse_hex(s) {
            return Ok(c);
        }
        return Err(ParseError::InvalidHex);
    }

    if let (Some(i), Some(s)) = (s.find('('), s.strip_suffix(")")) {
        let fname = &s[..i].trim_end();
        let s = &s[i + 1..].replace(",", " ").replace("/", " ");
        let params: Vec<&str> = s.split_whitespace().collect();
        let p_len = params.len();

        let mut a = Some(1.);

        if *fname == "rgb" || *fname == "rgba" {
            if p_len != 3 && p_len != 4 {
                return Err(ParseError::InvalidRgb);
            }
            let r = parse_percent_or_255(params[0]);
            let g = parse_percent_or_255(params[1]);
            let b = parse_percent_or_255(params[2]);
            if p_len == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if let (Some(r), Some(g), Some(b), Some(a)) = (r, g, b, a) {
                return Ok(Color {
                    r: clamp0_1(r),
                    g: clamp0_1(g),
                    b: clamp0_1(b),
                    a: clamp0_1(a),
                });
            }
            return Err(ParseError::InvalidRgb);
        } else if *fname == "hsl" || *fname == "hsla" {
            if p_len != 3 && p_len != 4 {
                return Err(ParseError::InvalidHsl);
            }
            let h = parse_angle(params[0]);
            let s = parse_percent_or_float(params[1]);
            let l = parse_percent_or_float(params[2]);
            if p_len == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if let (Some(h), Some(s), Some(l), Some(a)) = (h, s, l, a) {
                return Ok(Color::from_hsla(h, s, l, a));
            }
            return Err(ParseError::InvalidHsl);
        } else if *fname == "hwb" || *fname == "hwba" {
            if p_len != 3 && p_len != 4 {
                return Err(ParseError::InvalidHwb);
            }
            let h = parse_angle(params[0]);
            let w = parse_percent_or_float(params[1]);
            let b = parse_percent_or_float(params[2]);
            if p_len == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if let (Some(h), Some(w), Some(b), Some(a)) = (h, w, b, a) {
                return Ok(Color::from_hwba(h, w, b, a));
            }
            return Err(ParseError::InvalidHwb);
        } else if *fname == "hsv" || *fname == "hsva" {
            if p_len != 3 && p_len != 4 {
                return Err(ParseError::InvalidHsv);
            }
            let h = parse_angle(params[0]);
            let s = parse_percent_or_float(params[1]);
            let v = parse_percent_or_float(params[2]);
            if p_len == 4 {
                a = parse_percent_or_float(params[3]);
            }
            if let (Some(h), Some(s), Some(v), Some(a)) = (h, s, v, a) {
                return Ok(Color::from_hsva(h, s, v, a));
            }
            return Err(ParseError::InvalidHsv);
        }
    }

    if let Ok(c) = parse_hex(&s) {
        return Ok(c);
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
    Ok(Color::from_rgba_u8(r, g, b, a))
}

fn hue_to_rgb(n1: f64, n2: f64, h: f64) -> f64 {
    let h = modulo(h, 6.);
    if h < 1. {
        return n1 + ((n2 - n1) * h);
    }
    if h < 3. {
        return n2;
    }
    if h < 4. {
        return n1 + ((n2 - n1) * (4. - h));
    }
    n1
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0. {
        return (l, l, l);
    }
    let n2;
    if l < 0.5 {
        n2 = l * (1. + s);
    } else {
        n2 = l + s - (l * s);
    }
    let n1 = 2. * l - n2;
    let h = h / 60.;
    let r = hue_to_rgb(n1, n2, h + 2.);
    let g = hue_to_rgb(n1, n2, h);
    let b = hue_to_rgb(n1, n2, h - 2.);
    (r, g, b)
}

fn hwb_to_rgb(hue: f64, white: f64, black: f64) -> (f64, f64, f64) {
    if white + black >= 1. {
        let l = white / (white + black);
        return (l, l, l);
    }
    let (r, g, b) = hsl_to_rgb(hue, 1., 0.5);
    let r = r * (1. - white - black) + white;
    let g = g * (1. - white - black) + white;
    let b = b * (1. - white - black) + white;
    (r, g, b)
}

#[allow(clippy::float_cmp)]
fn hsv_to_hsl(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let mut s = s;
    let l = (2. - s) * v / 2.;
    if l != 0. {
        if l == 1. {
            s = 0.;
        } else if l < 0.5 {
            s = s * v / (l * 2.);
        } else {
            s = s * v / (2. - l * 2.);
        }
    }
    (h, s, l)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let (h, s, l) = hsv_to_hsl(h, s, v);
    hsl_to_rgb(h, s, l)
}

#[allow(clippy::float_cmp)]
fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let v = r.max(g.max(b));
    let d = v - r.min(g.min(b));
    if d == 0. {
        return (0., 0., v);
    }
    let s = d / v;
    let dr = (v - r) / d;
    let dg = (v - g) / d;
    let db = (v - b) / d;
    let mut h;
    if r == v {
        h = db - dg;
    } else if g == v {
        h = 2. + dr - db;
    } else {
        h = 4. + dg - dr;
    }
    h = (h * 60.) % 360.;
    (normalize_angle(h), s, v)
}

#[allow(clippy::float_cmp)]
fn rgb_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let l = (max + min) / 2.;
    if min == max {
        return (0., 0., l);
    }
    let d = max - min;
    let s;
    if l < 0.5 {
        s = d / (max + min);
    } else {
        s = d / (2. - max - min);
    }
    let dr = (max - r) / d;
    let dg = (max - g) / d;
    let db = (max - b) / d;
    let mut h;
    if r == max {
        h = db - dg;
    } else if g == max {
        h = 2. + dr - db;
    } else {
        h = 4. + dg - dr;
    }
    h = (h * 60.) % 360.;
    (normalize_angle(h), s, l)
}

fn rgb_to_hwb(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (hue, _, _) = rgb_to_hsl(r, g, b);
    let white = r.min(g.min(b));
    let black = 1. - r.max(g.max(b));
    (hue, white, black)
}

fn parse_percent_or_float(s: &str) -> Option<f64> {
    if let Some(s) = s.strip_suffix("%") {
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
    if let Some(s) = s.strip_suffix("%") {
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
    if let Some(s) = s.strip_suffix("deg") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t);
        }
        return None;
    }
    if let Some(s) = s.strip_suffix("grad") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 360. / 400.);
        }
        return None;
    }
    if let Some(s) = s.strip_suffix("rad") {
        if let Ok(t) = s.parse::<f64>() {
            return Some(t * 180. / PI);
        }
        return None;
    }
    if let Some(s) = s.strip_suffix("turn") {
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
    let mut t = t % 360.;
    if t < 0. {
        t += 360.;
    }
    t
}

fn interp_angle(a0: f64, a1: f64, t: f64) -> f64 {
    let delta = (((a1 - a0) % 360.) + 540.) % 360. - 180.;
    (a0 + t * delta + 360.) % 360.
}

fn clamp0_1(t: f64) -> f64 {
    if t < 0. {
        return 0.;
    }
    if t > 1. {
        return 1.;
    }
    t
}

fn modulo(x: f64, n: f64) -> f64 {
    (x % n + n) % n
}

// https://www.w3.org/TR/css-color-4/#named-colors

static NAMED_COLORS: phf::Map<&'static str, [u8; 3]> = phf_map! {
    "aliceblue" => [240, 248, 255],
    "antiquewhite" => [250, 235, 215],
    "aqua" => [0, 255, 255],
    "aquamarine" => [127, 255, 212],
    "azure" => [240, 255, 255],
    "beige" => [245, 245, 220],
    "bisque" => [255, 228, 196],
    "black" => [0, 0, 0],
    "blanchedalmond" => [255, 235, 205],
    "blue" => [0, 0, 255],
    "blueviolet" => [138, 43, 226],
    "brown" => [165, 42, 42],
    "burlywood" => [222, 184, 135],
    "cadetblue" => [95, 158, 160],
    "chartreuse" => [127, 255, 0],
    "chocolate" => [210, 105, 30],
    "coral" => [255, 127, 80],
    "cornflowerblue" => [100, 149, 237],
    "cornsilk" => [255, 248, 220],
    "crimson" => [220, 20, 60],
    "cyan" => [0, 255, 255],
    "darkblue" => [0, 0, 139],
    "darkcyan" => [0, 139, 139],
    "darkgoldenrod" => [184, 134, 11],
    "darkgray" => [169, 169, 169],
    "darkgreen" => [0, 100, 0],
    "darkgrey" => [169, 169, 169],
    "darkkhaki" => [189, 183, 107],
    "darkmagenta" => [139, 0, 139],
    "darkolivegreen" => [85, 107, 47],
    "darkorange" => [255, 140, 0],
    "darkorchid" => [153, 50, 204],
    "darkred" => [139, 0, 0],
    "darksalmon" => [233, 150, 122],
    "darkseagreen" => [143, 188, 143],
    "darkslateblue" => [72, 61, 139],
    "darkslategray" => [47, 79, 79],
    "darkslategrey" => [47, 79, 79],
    "darkturquoise" => [0, 206, 209],
    "darkviolet" => [148, 0, 211],
    "deeppink" => [255, 20, 147],
    "deepskyblue" => [0, 191, 255],
    "dimgray" => [105, 105, 105],
    "dimgrey" => [105, 105, 105],
    "dodgerblue" => [30, 144, 255],
    "firebrick" => [178, 34, 34],
    "floralwhite" => [255, 250, 240],
    "forestgreen" => [34, 139, 34],
    "fuchsia" => [255, 0, 255],
    "gainsboro" => [220, 220, 220],
    "ghostwhite" => [248, 248, 255],
    "gold" => [255, 215, 0],
    "goldenrod" => [218, 165, 32],
    "gray" => [128, 128, 128],
    "green" => [0, 128, 0],
    "greenyellow" => [173, 255, 47],
    "grey" => [128, 128, 128],
    "honeydew" => [240, 255, 240],
    "hotpink" => [255, 105, 180],
    "indianred" => [205, 92, 92],
    "indigo" => [75, 0, 130],
    "ivory" => [255, 255, 240],
    "khaki" => [240, 230, 140],
    "lavender" => [230, 230, 250],
    "lavenderblush" => [255, 240, 245],
    "lawngreen" => [124, 252, 0],
    "lemonchiffon" => [255, 250, 205],
    "lightblue" => [173, 216, 230],
    "lightcoral" => [240, 128, 128],
    "lightcyan" => [224, 255, 255],
    "lightgoldenrodyellow" => [250, 250, 210],
    "lightgray" => [211, 211, 211],
    "lightgreen" => [144, 238, 144],
    "lightgrey" => [211, 211, 211],
    "lightpink" => [255, 182, 193],
    "lightsalmon" => [255, 160, 122],
    "lightseagreen" => [32, 178, 170],
    "lightskyblue" => [135, 206, 250],
    "lightslategray" => [119, 136, 153],
    "lightslategrey" => [119, 136, 153],
    "lightsteelblue" => [176, 196, 222],
    "lightyellow" => [255, 255, 224],
    "lime" => [0, 255, 0],
    "limegreen" => [50, 205, 50],
    "linen" => [250, 240, 230],
    "magenta" => [255, 0, 255],
    "maroon" => [128, 0, 0],
    "mediumaquamarine" => [102, 205, 170],
    "mediumblue" => [0, 0, 205],
    "mediumorchid" => [186, 85, 211],
    "mediumpurple" => [147, 112, 219],
    "mediumseagreen" => [60, 179, 113],
    "mediumslateblue" => [123, 104, 238],
    "mediumspringgreen" => [0, 250, 154],
    "mediumturquoise" => [72, 209, 204],
    "mediumvioletred" => [199, 21, 133],
    "midnightblue" => [25, 25, 112],
    "mintcream" => [245, 255, 250],
    "mistyrose" => [255, 228, 225],
    "moccasin" => [255, 228, 181],
    "navajowhite" => [255, 222, 173],
    "navy" => [0, 0, 128],
    "oldlace" => [253, 245, 230],
    "olive" => [128, 128, 0],
    "olivedrab" => [107, 142, 35],
    "orange" => [255, 165, 0],
    "orangered" => [255, 69, 0],
    "orchid" => [218, 112, 214],
    "palegoldenrod" => [238, 232, 170],
    "palegreen" => [152, 251, 152],
    "paleturquoise" => [175, 238, 238],
    "palevioletred" => [219, 112, 147],
    "papayawhip" => [255, 239, 213],
    "peachpuff" => [255, 218, 185],
    "peru" => [205, 133, 63],
    "pink" => [255, 192, 203],
    "plum" => [221, 160, 221],
    "powderblue" => [176, 224, 230],
    "purple" => [128, 0, 128],
    "rebeccapurple" => [102, 51, 153],
    "red" => [255, 0, 0],
    "rosybrown" => [188, 143, 143],
    "royalblue" => [65, 105, 225],
    "saddlebrown" => [139, 69, 19],
    "salmon" => [250, 128, 114],
    "sandybrown" => [244, 164, 96],
    "seagreen" => [46, 139, 87],
    "seashell" => [255, 245, 238],
    "sienna" => [160, 82, 45],
    "silver" => [192, 192, 192],
    "skyblue" => [135, 206, 235],
    "slateblue" => [106, 90, 205],
    "slategray" => [112, 128, 144],
    "slategrey" => [112, 128, 144],
    "snow" => [255, 250, 250],
    "springgreen" => [0, 255, 127],
    "steelblue" => [70, 130, 180],
    "tan" => [210, 180, 140],
    "teal" => [0, 128, 128],
    "thistle" => [216, 191, 216],
    "tomato" => [255, 99, 71],
    "turquoise" => [64, 224, 208],
    "violet" => [238, 130, 238],
    "wheat" => [245, 222, 179],
    "white" => [255, 255, 255],
    "whitesmoke" => [245, 245, 245],
    "yellow" => [255, 255, 0],
    "yellowgreen" => [154, 205, 50],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_angle() {
        let data = vec![
            ("360", 360.),
            ("127.356", 127.356),
            ("+120deg", 120.),
            ("90deg", 90.),
            ("-127deg", -127.),
            ("100grad", 90.),
            ("1.5707963267948966rad", 90.),
            ("0.25turn", 90.),
            ("-0.25turn", -90.),
        ];
        for (s, expected) in data {
            let c = parse_angle(s);
            assert_eq!(Some(expected), c);
        }
    }

    #[test]
    fn test_normalize_angle() {
        let data = vec![
            (0., 0.),
            (360., 0.),
            (400., 40.),
            (1155., 75.),
            (-360., 0.),
            (-90., 270.),
            (-765., 315.),
        ];
        for (x, expected) in data {
            let c = normalize_angle(x);
            assert_eq!(expected, c);
        }
    }

    #[test]
    fn test_interp_angle() {
        let data = vec![
            ((0., 360., 0.5), 0.),
            ((360., 90., 0.), 0.),
            ((360., 90., 0.5), 45.),
            ((360., 90., 1.), 90.),
        ];
        for ((a, b, t), expected) in data {
            let v = interp_angle(a, b, t);
            assert_eq!(expected, v);
        }
    }
}
