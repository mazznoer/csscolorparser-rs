// Color deprecated methods

use crate::Color;

impl Color {
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

    #[deprecated = "Use [from_linear_rgba](#method.from_linear_rgba) instead."]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_linear_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_linear_rgba(r, g, b, 1.0)
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

    #[deprecated = "Use [from_hsva](#method.from_hsva) instead."]
    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        Self::from_hsva(h, s, v, 1.0)
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

    #[deprecated = "Use [from_hwba](#method.from_hwba) instead."]
    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    pub fn from_hwb(h: f32, w: f32, b: f32) -> Self {
        Self::from_hwba(h, w, b, 1.0)
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
    #[deprecated = "Use [to_laba](#method.to_laba) instead."]
    /// Returns: `[l, a, b, alpha]`
    pub fn to_lab(&self) -> [f32; 4] {
        self.to_laba()
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
    #[deprecated = "Use [to_lcha](#method.to_lcha) instead."]
    /// Returns: `[l, c, h, alpha]`
    pub fn to_lch(&self) -> [f32; 4] {
        self.to_lcha()
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

    // --- Since version 0.7.2

    #[deprecated = "Use [to_css_hex](#method.to_css_hex) instead."]
    /// Get the RGB hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        self.to_css_hex()
    }

    #[deprecated = "Use [to_css_rgb](#method.to_css_rgb) instead."]
    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        self.to_css_rgb()
    }
}
