use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "rust-rgb")]
use rgb::{RGB, RGBA};

#[cfg(feature = "serde")]
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "lab")]
use lab::{LCh, Lab};

use crate::utils::*;
use crate::{parse, ParseColorError};

#[cfg(feature = "named-colors")]
use crate::NAMED_COLORS;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// The color
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alpha
    pub a: f32,
}

impl Color {
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * 255.0 + 0.5) as u8,
            (self.g * 255.0 + 0.5) as u8,
            (self.b * 255.0 + 0.5) as u8,
            (self.a * 255.0 + 0.5) as u8,
        ]
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..65535]
    pub fn to_rgba16(&self) -> [u16; 4] {
        [
            (self.r * 65535.0 + 0.5) as u16,
            (self.g * 65535.0 + 0.5) as u16,
            (self.b * 65535.0 + 0.5) as u16,
            (self.a * 65535.0 + 0.5) as u16,
        ]
    }

    /// Restricts R, G, B, A values to the range [0..1].
    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    #[deprecated = "Use [new](#method.new) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    #[deprecated = "Use [new](#method.new) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[deprecated = "Use [from_rgba8](#method.from_rgba8) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    #[deprecated = "Use [from_rgba8](#method.from_rgba8) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    #[deprecated = "Use [from_linear_rgba](#method.from_linear_rgba) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_linear_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_linear_rgba(r, g, b, 1.0)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        fn from_linear(x: f32) -> f32 {
            if x >= 0.0031308 {
                return 1.055 * x.powf(1.0 / 2.4) - 0.055;
            }
            12.92 * x
        }
        Self::new(from_linear(r), from_linear(g), from_linear(b), a)
    }

    #[deprecated = "Use [from_linear_rgba8](#method.from_linear_rgba8) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_linear_rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::from_linear_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }

    #[deprecated = "Use [from_linear_rgba8](#method.from_linear_rgba8) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_linear_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_linear_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_linear_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_linear_rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    #[deprecated = "Use [from_hsva](#method.from_hsva) instead."]
    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        Self::from_hsva(h, s, v, 1.0)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        let (r, g, b) = hsv_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(v));
        Self::new(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    #[deprecated = "Use [from_hsla](#method.from_hsla) instead."]
    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        Self::from_hsla(h, s, l, 1.0)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(l));
        Self::new(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    #[deprecated = "Use [from_hwba](#method.from_hwba) instead."]
    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    pub fn from_hwb(h: f32, w: f32, b: f32) -> Self {
        Self::from_hwba(h, w, b, 1.0)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hwba(h: f32, w: f32, b: f32, a: f32) -> Self {
        let (r, g, b) = hwb_to_rgb(normalize_angle(h), clamp0_1(w), clamp0_1(b));
        Self::new(clamp0_1(r), clamp0_1(g), clamp0_1(b), a)
    }

    #[deprecated = "Use [from_oklaba](#method.from_oklaba) instead."]
    /// Arguments:
    ///
    /// * `l`: Perceived lightness
    /// * `a`: How green/red the color is
    /// * `b`: How blue/yellow the color is
    pub fn from_oklab(l: f32, a: f32, b: f32) -> Self {
        Self::from_oklaba(l, a, b, 1.0)
    }

    /// Arguments:
    ///
    /// * `l`: Perceived lightness
    /// * `a`: How green/red the color is
    /// * `b`: How blue/yellow the color is
    /// * `alpha`: Alpha [0..1]
    #[allow(clippy::excessive_precision)]
    pub fn from_oklaba(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        let l_ = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
        let m_ = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
        let s_ = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);

        let r = 4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_;
        let g = -1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_;
        let b = -0.0041960863 * l_ - 0.7034186147 * m_ + 1.7076147010 * s_;

        Self::from_linear_rgba(r, g, b, alpha)
    }

    /// Arguments:
    ///
    /// * `l`: Perceived lightness
    /// * `c`: Chroma
    /// * `h`: Hue angle in radians
    /// * `alpha`: Alpha [0..1]
    pub fn from_oklcha(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        Self::from_oklaba(l, c * h.cos(), c * h.sin(), alpha)
    }

    #[cfg(feature = "lab")]
    /// Arguments:
    ///
    /// * `l`: Lightness
    /// * `a`: Distance along the `a` axis
    /// * `b`: Distance along the `b` axis
    /// * `alpha`: Alpha [0..1]
    pub fn from_laba(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        let [r, g, b] = Lab { l, a, b }.to_rgb_normalized();
        Self::new(r, g, b, alpha)
    }

    #[cfg(feature = "lab")]
    #[deprecated = "Use [from_laba](#method.from_laba) instead."]
    /// Arguments:
    ///
    /// * `l`: Lightness
    /// * `a`: Distance along the `a` axis
    /// * `b`: Distance along the `b` axis
    /// * `alpha`: Alpha [0..1]
    pub fn from_lab(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self::from_laba(l, a, b, alpha)
    }

    #[cfg(feature = "lab")]
    /// Returns: `[l, a, b, alpha]`
    pub fn to_laba(&self) -> [f32; 4] {
        let Lab { l, a, b } = Lab::from_rgb_normalized(&[self.r, self.g, self.b]);
        [l, a, b, self.a]
    }

    #[cfg(feature = "lab")]
    #[deprecated = "Use [to_laba](#method.to_laba) instead."]
    /// Returns: `[l, a, b, alpha]`
    pub fn to_lab(&self) -> [f32; 4] {
        self.to_laba()
    }

    #[cfg(feature = "lab")]
    /// Blend this color with the other one, in the Lab color-space. `t` in the range [0..1].
    pub fn interpolate_lab(&self, other: &Color, t: f32) -> Self {
        let [l1, a1, b1, alpha1] = self.to_laba();
        let [l2, a2, b2, alpha2] = other.to_laba();
        Self::from_laba(
            l1 + t * (l2 - l1),
            a1 + t * (a2 - a1),
            b1 + t * (b2 - b1),
            alpha1 + t * (alpha2 - alpha1),
        )
    }

    #[cfg(feature = "lab")]
    /// Arguments:
    ///
    /// * `l`: Lightness
    /// * `c`: Chroma
    /// * `h`: Hue angle in radians
    /// * `alpha`: Alpha [0..1]
    pub fn from_lcha(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        let [r, g, b] = LCh { l, c, h }.to_lab().to_rgb_normalized();
        Self::new(r, g, b, alpha)
    }

    #[cfg(feature = "lab")]
    #[deprecated = "Use [from_lcha](#method.from_lcha) instead."]
    /// Arguments:
    ///
    /// * `l`: Lightness
    /// * `c`: Chroma
    /// * `h`: Hue angle in radians
    /// * `alpha`: Alpha [0..1]
    pub fn from_lch(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        Self::from_lcha(l, c, h, alpha)
    }

    #[cfg(feature = "lab")]
    /// Returns: `[l, c, h, alpha]`
    pub fn to_lcha(&self) -> [f32; 4] {
        let LCh { l, c, h } = LCh::from_lab(Lab::from_rgb_normalized(&[self.r, self.g, self.b]));
        [l, c, h, self.a]
    }

    #[cfg(feature = "lab")]
    #[deprecated = "Use [to_lcha](#method.to_lcha) instead."]
    /// Returns: `[l, c, h, alpha]`
    pub fn to_lch(&self) -> [f32; 4] {
        self.to_lcha()
    }

    #[cfg(feature = "lab")]
    /// Blend this color with the other one, in the LCH color-space. `t` in the range [0..1].
    pub fn interpolate_lch(&self, other: &Color, t: f32) -> Self {
        let [l1, c1, h1, alpha1] = self.to_lcha();
        let [l2, c2, h2, alpha2] = other.to_lcha();
        Self::from_lcha(
            l1 + t * (l2 - l1),
            c1 + t * (c2 - c1),
            interp_angle_rad(h1, h2, t),
            alpha1 + t * (alpha2 - alpha1),
        )
    }

    /// Create color from CSS color string.
    ///
    /// # Examples
    /// ```
    /// use csscolorparser::Color;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let c = Color::from_html("rgb(255,0,0)")?;
    ///
    /// assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
    /// assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    /// assert_eq!(c.to_hex_string(), "#ff0000");
    /// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_html<S: AsRef<str>>(s: S) -> Result<Self, ParseColorError> {
        parse(s.as_ref())
    }

    /// Returns name if there is a name for this color.
    ///
    /// **Note:** It ignores transparency (alpha value).
    ///
    /// ```
    /// use csscolorparser::Color;
    ///
    /// assert_eq!(Color::from_rgba8(255, 0, 0, 255).name(), Some("red"));
    /// assert_eq!(Color::from_rgba8(238, 130, 238, 255).name(), Some("violet"));
    /// assert_eq!(Color::from_rgba8(90, 150, 200, 255).name(), None);
    /// ```
    #[cfg(feature = "named-colors")]
    pub fn name(&self) -> Option<&'static str> {
        let rgb = &self.to_rgba8()[0..3];
        for (&k, &v) in NAMED_COLORS.entries() {
            if v == rgb {
                return Some(k);
            }
        }
        None
    }

    #[deprecated]
    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn rgba(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    #[deprecated = "Use [to_rgba8](#method.to_rgba8) instead."]
    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
            (self.a * 255.0).round() as u8,
        )
    }

    /// Returns: `[h, s, v, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsva(&self) -> [f32; 4] {
        let (h, s, v) = rgb_to_hsv(self.r, self.g, self.b);
        [h, s, v, self.a]
    }

    /// Returns: `[h, s, l, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> [f32; 4] {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        [h, s, l, self.a]
    }

    /// Returns: `[h, w, b, a]`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hwba(&self) -> [f32; 4] {
        let (h, w, b) = rgb_to_hwb(self.r, self.g, self.b);
        [h, w, b, self.a]
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_linear_rgba(&self) -> [f32; 4] {
        fn to_linear(x: f32) -> f32 {
            if x >= 0.04045 {
                return ((x + 0.055) / 1.055).powf(2.4);
            }
            x / 12.92
        }
        [
            to_linear(self.r),
            to_linear(self.g),
            to_linear(self.b),
            self.a,
        ]
    }

    /// Returns: `[r, g, b, a]`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn to_linear_rgba_u8(&self) -> [u8; 4] {
        let [r, g, b, a] = self.to_linear_rgba();
        [
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ]
    }

    /// Returns: `[l, a, b, alpha]`
    #[allow(clippy::excessive_precision)]
    pub fn to_oklaba(&self) -> [f32; 4] {
        let [r, g, b, _] = self.to_linear_rgba();
        let l_ = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
        let m_ = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
        let s_ = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
        let l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
        [l, a, b, self.a]
    }

    /// Get the RGB hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        let [r, g, b, a] = self.to_rgba8();

        if a < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);
        }

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        let [r, g, b, _] = self.to_rgba8();

        if self.a < 1.0 {
            return format!("rgba({},{},{},{})", r, g, b, self.a);
        }

        format!("rgb({},{},{})", r, g, b)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Color, t: f32) -> Self {
        Self {
            r: self.r + t * (other.r - self.r),
            g: self.g + t * (other.g - self.g),
            b: self.b + t * (other.b - self.b),
            a: self.a + t * (other.a - self.a),
        }
    }

    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_linear_rgb(&self, other: &Color, t: f32) -> Self {
        let [r1, g1, b1, a1] = self.to_linear_rgba();
        let [r2, g2, b2, a2] = other.to_linear_rgba();
        Self::from_linear_rgba(
            r1 + t * (r2 - r1),
            g1 + t * (g2 - g1),
            b1 + t * (b2 - b1),
            a1 + t * (a2 - a1),
        )
    }

    /// Blend this color with the other one, in the HSV color-space. `t` in the range [0..1].
    pub fn interpolate_hsv(&self, other: &Color, t: f32) -> Self {
        let [h1, s1, v1, a1] = self.to_hsva();
        let [h2, s2, v2, a2] = other.to_hsva();
        Self::from_hsva(
            interp_angle(h1, h2, t),
            s1 + t * (s2 - s1),
            v1 + t * (v2 - v1),
            a1 + t * (a2 - a1),
        )
    }

    /// Blend this color with the other one, in the [Oklab](https://bottosson.github.io/posts/oklab/) color-space. `t` in the range [0..1].
    pub fn interpolate_oklab(&self, other: &Color, t: f32) -> Self {
        let [l1, a1, b1, alpha1] = self.to_oklaba();
        let [l2, a2, b2, alpha2] = other.to_oklaba();
        Self::from_oklaba(
            l1 + t * (l2 - l1),
            a1 + t * (a2 - a1),
            b1 + t * (b2 - b1),
            alpha1 + t * (alpha2 - alpha1),
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGBA({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl TryFrom<&str> for Color {
    type Error = ParseColorError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse(s)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<[f64; 4]> for Color {
    fn from([r, g, b, a]: [f64; 4]) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: a as f32,
        }
    }
}

impl From<[f64; 3]> for Color {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self {
            r: r as f32,
            g: g as f32,
            b: b as f32,
            a: 1.0,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::from_rgba8(r, g, b, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::from_rgba8(r, g, b, a)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
}

/// Convert rust-rgb's `RGB<f32>` type into `Color`.
#[cfg(feature = "rust-rgb")]
impl From<RGB<f32>> for Color {
    fn from(item: RGB<f32>) -> Self {
        Self::new(item.r, item.g, item.b, 1.0)
    }
}

/// Convert rust-rgb's `RGBA<f32>` type into `Color`.
#[cfg(feature = "rust-rgb")]
impl From<RGBA<f32>> for Color {
    fn from(item: RGBA<f32>) -> Self {
        Self::new(item.r, item.g, item.b, item.a)
    }
}

/// Implement Serde serialization into HEX string
#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex_string())
    }
}

/// Implement Serde deserialization from string
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(ColorVisitor)
    }
}

#[cfg(feature = "serde")]
struct ColorVisitor;

#[cfg(feature = "serde")]
impl Visitor<'_> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid css color")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Color::from_str(v).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "rust-rgb")]
    #[test]
    fn test_convert_rust_rgb_to_color() {
        let rgb = RGB::new(0.0, 0.5, 1.0);
        assert_eq!(Color::new(0.0, 0.5, 1.0, 1.0), Color::from(rgb));

        let rgba = RGBA::new(1.0, 0.5, 0.0, 0.5);
        assert_eq!(Color::new(1.0, 0.5, 0.0, 0.5), Color::from(rgba));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialize_to_hex() {
        let color = Color::new(1.0, 1.0, 0.5, 0.5);
        serde_test::assert_ser_tokens(&color, &[serde_test::Token::Str("#ffff8080")]);
    }

    #[cfg(all(feature = "serde", feature = "named-colors"))]
    #[test]
    fn test_serde_deserialize_from_string() {
        let named = Color::new(1.0, 1.0, 0.0, 1.0);
        serde_test::assert_de_tokens(&named, &[serde_test::Token::Str("yellow")]);

        let hex = Color::new(0.0, 1.0, 0.0, 1.0);
        serde_test::assert_de_tokens(&hex, &[serde_test::Token::Str("#00ff00ff")]);

        let rgb = Color::new(0.0, 1.0, 0.0, 1.0);
        serde_test::assert_de_tokens(&rgb, &[serde_test::Token::Str("rgba(0,255,0,1)")]);
    }
}
