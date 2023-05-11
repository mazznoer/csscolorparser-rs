use crate::Color;
use cint::{Alpha, ColorInterop, EncodedSrgb};

impl ColorInterop for Color {
    type CintTy = Alpha<EncodedSrgb<f32>>;
}

impl From<Color> for EncodedSrgb<f32> {
    fn from(c: Color) -> Self {
        let Color { r, g, b, a: _ } = c;
        EncodedSrgb { r, g, b }
    }
}

impl From<EncodedSrgb<f32>> for Color {
    fn from(c: EncodedSrgb<f32>) -> Self {
        let EncodedSrgb { r, g, b } = c;
        Self::new(r, g, b, 1.0)
    }
}

impl From<Color> for EncodedSrgb<f64> {
    fn from(c: Color) -> Self {
        let Color { r, g, b, a: _ } = c;
        let (r, g, b) = (r as f64, g as f64, b as f64);
        EncodedSrgb { r, g, b }
    }
}

impl From<EncodedSrgb<f64>> for Color {
    fn from(c: EncodedSrgb<f64>) -> Self {
        let EncodedSrgb { r, g, b } = c;
        let (r, g, b) = (r as f32, g as f32, b as f32);
        Self::new(r, g, b, 1.0)
    }
}

impl From<Color> for Alpha<EncodedSrgb<f32>> {
    fn from(c: Color) -> Self {
        let Color { r, g, b, a } = c;
        Alpha {
            color: EncodedSrgb { r, g, b },
            alpha: a,
        }
    }
}

impl From<Alpha<EncodedSrgb<f32>>> for Color {
    fn from(c: Alpha<EncodedSrgb<f32>>) -> Self {
        let Alpha {
            color: EncodedSrgb { r, g, b },
            alpha,
        } = c;
        Self::new(r, g, b, alpha)
    }
}

impl From<Color> for Alpha<EncodedSrgb<f64>> {
    fn from(c: Color) -> Self {
        let Color { r, g, b, a } = c;
        let (r, g, b, alpha) = (r as f64, g as f64, b as f64, a as f64);
        Alpha {
            color: EncodedSrgb { r, g, b },
            alpha,
        }
    }
}

impl From<Alpha<EncodedSrgb<f64>>> for Color {
    fn from(c: Alpha<EncodedSrgb<f64>>) -> Self {
        let Alpha {
            color: EncodedSrgb { r, g, b },
            alpha,
        } = c;
        let (r, g, b, alpha) = (r as f32, g as f32, b as f32, alpha as f32);
        Self::new(r, g, b, alpha)
    }
}

impl From<Color> for EncodedSrgb<u8> {
    fn from(c: Color) -> Self {
        let [r, g, b, _] = c.to_rgba8();
        EncodedSrgb { r, g, b }
    }
}

impl From<EncodedSrgb<u8>> for Color {
    fn from(c: EncodedSrgb<u8>) -> Self {
        let EncodedSrgb { r, g, b } = c;
        Self::from_rgba8(r, g, b, 255)
    }
}

impl From<Color> for Alpha<EncodedSrgb<u8>> {
    fn from(c: Color) -> Self {
        let [r, g, b, alpha] = c.to_rgba8();
        Alpha {
            color: EncodedSrgb { r, g, b },
            alpha,
        }
    }
}

impl From<Alpha<EncodedSrgb<u8>>> for Color {
    fn from(c: Alpha<EncodedSrgb<u8>>) -> Self {
        let Alpha {
            color: EncodedSrgb { r, g, b },
            alpha,
        } = c;
        Self::from_rgba8(r, g, b, alpha)
    }
}
