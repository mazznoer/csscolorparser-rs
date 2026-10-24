#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Conversion between linear-light sRGB and A98 RGB (Adobe RGB 1998),
// matching the `a98-rgb` color space from CSS Color Level 4.
//
// Notes:
// - Inputs/outputs are not clamped, so out-of-gamut colors come back as
//   values below 0.0 or above 1.0.
// - Both spaces use the D65 white point, so no chromatic adaptation is needed.

/// A98 RGB transfer function exponent (563/256 ≈ 2.19921875).
const A98_GAMMA: f32 = 563.0 / 256.0;

/// Linear A98 RGB -> linear sRGB.
///
/// Equivalent to `XYZ_TO_LIN_SRGB * LIN_A98_TO_XYZ` using the matrices from
/// the CSS Color 4 spec. The two spaces share the red and blue primaries and
/// the white point, which is why several entries are exactly zero.
const LIN_A98_TO_LIN_SRGB: [[f32; 3]; 3] = [
    [1.3983557, -0.39835575, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, -0.04292899, 1.0429289],
];

/// Linear sRGB -> linear A98 RGB (inverse of the matrix above).
const LIN_SRGB_TO_LIN_A98: [[f32; 3]; 3] = [
    [0.7151256, 0.28487438, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.041161947, 0.95883805],
];

#[inline]
fn mul3(m: &[[f32; 3]; 3], v: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Gamma-encoded A98 RGB -> linear light (sign-preserving power function).
#[inline]
fn a98_decode(v: f32) -> f32 {
    v.abs().powf(A98_GAMMA).copysign(v)
}

/// Linear light -> gamma-encoded A98 RGB (sign-preserving power function).
#[inline]
fn a98_encode(v: f32) -> f32 {
    v.abs().powf(1.0 / A98_GAMMA).copysign(v)
}

/// Converts a gamma-encoded A98 RGB color to linear sRGB.
pub(crate) fn a98_to_linear_srgb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let linear_a98 = [a98_decode(r), a98_decode(g), a98_decode(b)];
    mul3(&LIN_A98_TO_LIN_SRGB, linear_a98)
}

/// Converts a linear sRGB color to gamma-encoded A98 RGB.
pub(crate) fn linear_srgb_to_a98(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [lr, lg, lb] = mul3(&LIN_SRGB_TO_LIN_A98, [r, g, b]);
    [a98_encode(lr), a98_encode(lg), a98_encode(lb)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: [f32; 3], b: [f32; 3], eps: f32) {
        for i in 0..3 {
            assert!((a[i] - b[i]).abs() <= eps, "{a:?} != {b:?} (eps {eps})");
        }
    }

    #[test]
    fn black_and_white_map_to_themselves() {
        assert_close(a98_to_linear_srgb(0.0, 0.0, 0.0), [0.0; 3], 1e-6);
        assert_close(a98_to_linear_srgb(1.0, 1.0, 1.0), [1.0; 3], 1e-6);
        assert_close(linear_srgb_to_a98(0.0, 0.0, 0.0), [0.0; 3], 1e-6);
        assert_close(linear_srgb_to_a98(1.0, 1.0, 1.0), [1.0; 3], 1e-6);
    }

    #[test]
    fn grays_stay_neutral() {
        // 0.5^(563/256) ≈ 0.21775553
        assert_close(a98_to_linear_srgb(0.5, 0.5, 0.5), [0.21775553; 3], 1e-6);
    }

    #[test]
    fn a98_primaries_in_linear_srgb() {
        assert_close(
            a98_to_linear_srgb(1.0, 0.0, 0.0),
            [1.3983557, 0.0, 0.0],
            1e-6,
        );
        // A98 green is outside the sRGB gamut -> negative components.
        assert_close(
            a98_to_linear_srgb(0.0, 1.0, 0.0),
            [-0.39835575, 1.0, -0.04292899],
            1e-6,
        );
    }

    #[test]
    fn round_trip() {
        for &(r, g, b) in &[(0.2, 0.5, 0.8), (0.9, 0.1, 0.3), (0.05, 0.95, 0.5)] {
            let lin = a98_to_linear_srgb(r, g, b);
            let back = linear_srgb_to_a98(lin[0], lin[1], lin[2]);
            assert_close(back, [r, g, b], 1e-5);
        }
    }
}
