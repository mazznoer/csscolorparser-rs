#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Conversions between linear sRGB and CSS `display-p3`.
//
// CSS Color 4 `color(display-p3 r g b)` is *gamma-encoded* (it uses the sRGB
// transfer function), so:
//
//   display-p3 (encoded) -> linearize -> 3x3 matrix -> linear sRGB
//   linear sRGB -> 3x3 matrix -> linear P3 -> sRGB transfer function -> display-p3 (encoded)
//
// Out-of-gamut values (negative or > 1) are preserved, not clamped, and the
// transfer function is mirrored around zero as the CSS spec does.

/// Linear Display-P3 -> linear sRGB (both D65).
const P3_TO_SRGB: [[f32; 3]; 3] = [
    [1.224_940_2, -0.224_940_18, 0.0],
    [-0.042_056_955, 1.042_056_9, 0.0],
    [-0.019_637_555, -0.078_636_045, 1.098_273_6],
];

/// Linear sRGB -> linear Display-P3 (both D65).
const SRGB_TO_P3: [[f32; 3]; 3] = [
    [0.822_461_97, 0.177_538_03, 0.0],
    [0.033_194_2, 0.966_805_8, 0.0],
    [0.017_082_63, 0.072_397_44, 0.910_519_9],
];

#[inline]
fn mul3(m: &[[f32; 3]; 3], r: f32, g: f32, b: f32) -> [f32; 3] {
    [
        m[0][0] * r + m[0][1] * g + m[0][2] * b,
        m[1][0] * r + m[1][1] * g + m[1][2] * b,
        m[2][0] * r + m[2][1] * g + m[2][2] * b,
    ]
}

/// sRGB electro-optical transfer function (gamma-encoded -> linear).
#[inline]
fn srgb_to_linear(c: f32) -> f32 {
    let a = c.abs();
    let l = if a <= 0.04045 {
        a / 12.92
    } else {
        ((a + 0.055) / 1.055).powf(2.4)
    };
    l.copysign(c)
}

/// sRGB inverse transfer function (linear -> gamma-encoded).
#[inline]
fn linear_to_srgb(c: f32) -> f32 {
    let a = c.abs();
    let e = if a <= 0.003_130_8 {
        a * 12.92
    } else {
        1.055 * a.powf(1.0 / 2.4) - 0.055
    };
    e.copysign(c)
}

/// Converts a CSS `display-p3` color (gamma-encoded, components nominally 0..=1)
/// to linear sRGB. Results may fall outside 0..=1 for colors outside the sRGB gamut.
pub(crate) fn display_p3_to_linear_srgb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let (r, g, b) = (srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));
    mul3(&P3_TO_SRGB, r, g, b)
}

/// Converts a linear sRGB color to a CSS `display-p3` color (gamma-encoded).
/// Results may fall outside 0..=1 for colors outside the P3 gamut.
pub(crate) fn linear_srgb_to_display_p3(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [r, g, b] = mul3(&SRGB_TO_P3, r, g, b);
    [linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: [f32; 3], b: [f32; 3], eps: f32) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() <= eps)
    }

    #[test]
    fn white_and_black_are_fixed_points() {
        assert!(close(
            display_p3_to_linear_srgb(1.0, 1.0, 1.0),
            [1.0, 1.0, 1.0],
            1e-5
        ));
        assert!(close(
            display_p3_to_linear_srgb(0.0, 0.0, 0.0),
            [0.0, 0.0, 0.0],
            1e-6
        ));
        assert!(close(
            linear_srgb_to_display_p3(1.0, 1.0, 1.0),
            [1.0, 1.0, 1.0],
            1e-5
        ));
    }

    #[test]
    fn p3_red_is_outside_srgb() {
        // color(display-p3 1 0 0) -> linear sRGB ~ (1.2249, -0.0421, -0.0196)
        let out = display_p3_to_linear_srgb(1.0, 0.0, 0.0);
        assert!(close(out, [1.224_94, -0.042_057, -0.019_638], 1e-4));
    }

    #[test]
    fn round_trip() {
        for &(r, g, b) in &[
            (0.2, 0.5, 0.8),
            (1.0, 0.0, 0.0),
            (0.01, 0.9, 0.3),
            (-0.1, 0.4, 1.1),
        ] {
            let lin = display_p3_to_linear_srgb(r, g, b);
            let back = linear_srgb_to_display_p3(lin[0], lin[1], lin[2]);
            assert!(
                close(back, [r, g, b], 1e-4),
                "{:?} vs {:?}",
                back,
                [r, g, b]
            );
        }
    }
}
