#![allow(clippy::excessive_precision)]

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// sRGB <-> ProPhoto RGB conversion, following the sample code in
// CSS Color Module Level 4 (https://www.w3.org/TR/css-color-4/#color-conversion-code).
//
// Pipeline (sRGB -> ProPhoto):
//   gamma-encoded sRGB -> linear sRGB -> XYZ (D65) -> XYZ (D50, Bradford)
//   -> linear ProPhoto -> gamma-encoded ProPhoto
//
// Inputs/outputs are gamma-encoded components, nominally in 0.0..=1.0.
// As in CSS, values are NOT clamped: out-of-gamut colors produce components
// outside 0..=1 (and negative values are handled symmetrically).

type Mat3 = [[f64; 3]; 3];

// ---------------------------------------------------------------------------
// Matrices (from the CSS Color 4 spec)
// ---------------------------------------------------------------------------

/// Linear sRGB -> XYZ (D65)
const LIN_SRGB_TO_XYZ_D65: Mat3 = [
    [506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
    [87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
    [7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
];

/// XYZ (D65) -> linear sRGB
const XYZ_D65_TO_LIN_SRGB: Mat3 = [
    [12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
    [
        -851781.0 / 878810.0,
        1648619.0 / 878810.0,
        36519.0 / 878810.0,
    ],
    [705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
];

/// Bradford chromatic adaptation, D65 -> D50
const D65_TO_D50: Mat3 = [
    [
        1.0479297925449969,
        0.022946870601609652,
        -0.05019226628920524,
    ],
    [
        0.02962780877005599,
        0.9904344267538799,
        -0.017073799063418826,
    ],
    [
        -0.009243040646204504,
        0.015055191490298152,
        0.7518742814281371,
    ],
];

/// Bradford chromatic adaptation, D50 -> D65
const D50_TO_D65: Mat3 = [
    [
        0.9554734527042182,
        -0.023098536874261423,
        0.0632593086610217,
    ],
    [
        -0.028369706963208136,
        1.0099954580058226,
        0.021041398966943008,
    ],
    [
        0.012314001688319899,
        -0.020507696433477912,
        1.3303659366080753,
    ],
];

/// Linear ProPhoto -> XYZ (D50)
const LIN_PROPHOTO_TO_XYZ_D50: Mat3 = [
    [
        0.79776664490064230,
        0.13518129740053308,
        0.03134773412839220,
    ],
    [
        0.28807482881940130,
        0.71183523424187300,
        0.00008993693872564,
    ],
    [
        0.00000000000000000,
        0.00000000000000000,
        0.82510460251046020,
    ],
];

/// XYZ (D50) -> linear ProPhoto
const XYZ_D50_TO_LIN_PROPHOTO: Mat3 = [
    [
        1.34578688164715830,
        -0.25557208737979464,
        -0.05110186497554526,
    ],
    [
        -0.54463070512490190,
        1.50824774284514680,
        0.02052744743642139,
    ],
    [
        0.00000000000000000,
        0.00000000000000000,
        1.21196754563894520,
    ],
];

// Helpers

#[inline]
fn mul(m: &Mat3, v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// sRGB gamma-encoded -> linear light (sign-preserving for out-of-range input)
#[inline]
fn srgb_to_linear(v: f64) -> f64 {
    let a = v.abs();
    if a <= 0.04045 {
        v / 12.92
    } else {
        v.signum() * ((a + 0.055) / 1.055).powf(2.4)
    }
}

/// linear light -> sRGB gamma-encoded (sign-preserving)
#[inline]
fn linear_to_srgb(v: f64) -> f64 {
    let a = v.abs();
    if a > 0.0031308 {
        v.signum() * (1.055 * a.powf(1.0 / 2.4) - 0.055)
    } else {
        12.92 * v
    }
}

/// ProPhoto gamma-encoded -> linear light (sign-preserving)
#[inline]
fn prophoto_to_linear(v: f64) -> f64 {
    const ET2: f64 = 16.0 / 512.0;
    let a = v.abs();
    if a <= ET2 {
        v / 16.0
    } else {
        v.signum() * a.powf(1.8)
    }
}

/// linear light -> ProPhoto gamma-encoded (sign-preserving)
#[inline]
fn linear_to_prophoto(v: f64) -> f64 {
    const ET: f64 = 1.0 / 512.0;
    let a = v.abs();
    if a >= ET {
        v.signum() * a.powf(1.0 / 1.8)
    } else {
        16.0 * v
    }
}

/// Convert gamma-encoded sRGB to gamma-encoded ProPhoto RGB.
pub(crate) fn srgb_to_prophoto(r: f32, g: f32, b: f32) -> [f32; 3] {
    let lin = [
        srgb_to_linear(r as f64),
        srgb_to_linear(g as f64),
        srgb_to_linear(b as f64),
    ];
    let xyz_d65 = mul(&LIN_SRGB_TO_XYZ_D65, lin);
    let xyz_d50 = mul(&D65_TO_D50, xyz_d65);
    let pp = mul(&XYZ_D50_TO_LIN_PROPHOTO, xyz_d50);
    [
        linear_to_prophoto(pp[0]) as f32,
        linear_to_prophoto(pp[1]) as f32,
        linear_to_prophoto(pp[2]) as f32,
    ]
}

/// Convert gamma-encoded ProPhoto RGB to gamma-encoded sRGB.
///
/// The result may fall outside 0.0..=1.0 for colors outside the sRGB gamut.
pub(crate) fn prophoto_to_srgb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let lin = [
        prophoto_to_linear(r as f64),
        prophoto_to_linear(g as f64),
        prophoto_to_linear(b as f64),
    ];
    let xyz_d50 = mul(&LIN_PROPHOTO_TO_XYZ_D50, lin);
    let xyz_d65 = mul(&D50_TO_D65, xyz_d50);
    let srgb = mul(&XYZ_D65_TO_LIN_SRGB, xyz_d65);
    [
        linear_to_srgb(srgb[0]) as f32,
        linear_to_srgb(srgb[1]) as f32,
        linear_to_srgb(srgb[2]) as f32,
    ]
}
