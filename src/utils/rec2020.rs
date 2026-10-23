#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// sRGB <-> Rec.2020 (ITU-R BT.2020) conversion.
//
// Inputs and outputs are gamma-encoded r, g, b components, nominal range 0.0..=1.0.
// Both spaces use D65 white, so no chromatic adaptation is needed.
//
// `rec2020_to_srgb` can return values below 0 or above 1 for colours outside
// the sRGB gamut. The result is not clipped, so the conversion stays invertible.

/// Linear sRGB -> linear Rec.2020 (D65).
const SRGB_TO_REC2020: [[f64; 3]; 3] = [
    [
        0.627_403_895_934_699_1,
        0.329_283_038_377_883_7,
        0.043_313_065_687_417_23,
    ],
    [
        0.069_097_289_358_232_08,
        0.919_540_395_075_458_8,
        0.011_362_315_566_309_18,
    ],
    [
        0.016_391_438_875_150_28,
        0.088_013_307_877_225_76,
        0.895_595_253_247_624,
    ],
];

/// Linear Rec.2020 -> linear sRGB (D65).
const REC2020_TO_SRGB: [[f64; 3]; 3] = [
    [
        1.660_491_002_108_434_4,
        -0.587_641_138_788_549_5,
        -0.072_849_863_319_884_88,
    ],
    [
        -0.124_550_474_521_590_74,
        1.132_899_897_125_960_2,
        -0.008_349_422_604_369_477,
    ],
    [
        -0.018_150_763_354_905_3,
        -0.100_578_898_008_007_38,
        1.118_729_661_362_912_7,
    ],
];

const BT2020_ALPHA: f64 = 1.099_296_826_809_44;
const BT2020_BETA: f64 = 0.018_053_968_510_807;

/// Convert gamma-encoded sRGB to gamma-encoded Rec.2020.
pub(crate) fn srgb_to_rec2020(r: f64, g: f64, b: f64) -> [f64; 3] {
    let (r, g, b) = mul(
        &SRGB_TO_REC2020,
        srgb_decode(r),
        srgb_decode(g),
        srgb_decode(b),
    );
    [rec2020_encode(r), rec2020_encode(g), rec2020_encode(b)]
}

/// Convert gamma-encoded Rec.2020 to gamma-encoded sRGB.
pub(crate) fn rec2020_to_srgb(r: f64, g: f64, b: f64) -> [f64; 3] {
    let (r, g, b) = mul(
        &REC2020_TO_SRGB,
        rec2020_decode(r),
        rec2020_decode(g),
        rec2020_decode(b),
    );
    [srgb_encode(r), srgb_encode(g), srgb_encode(b)]
}

fn mul(m: &[[f64; 3]; 3], r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    (
        m[0][0] * r + m[0][1] * g + m[0][2] * b,
        m[1][0] * r + m[1][1] * g + m[1][2] * b,
        m[2][0] * r + m[2][1] * g + m[2][2] * b,
    )
}

// Transfer functions mirror around zero so out-of-gamut (negative) values
// survive the round trip.

fn srgb_decode(v: f64) -> f64 {
    let a = v.abs();
    let lin = if a <= 0.04045 {
        a / 12.92
    } else {
        ((a + 0.055) / 1.055).powf(2.4)
    };
    lin.copysign(v)
}

fn srgb_encode(l: f64) -> f64 {
    let a = l.abs();
    let enc = if a <= 0.003_130_8 {
        12.92 * a
    } else {
        1.055 * a.powf(1.0 / 2.4) - 0.055
    };
    enc.copysign(l)
}

fn rec2020_decode(v: f64) -> f64 {
    let a = v.abs();
    let lin = if a < 4.5 * BT2020_BETA {
        a / 4.5
    } else {
        ((a + BT2020_ALPHA - 1.0) / BT2020_ALPHA).powf(1.0 / 0.45)
    };
    lin.copysign(v)
}

fn rec2020_encode(l: f64) -> f64 {
    let a = l.abs();
    let enc = if a < BT2020_BETA {
        4.5 * a
    } else {
        BT2020_ALPHA * a.powf(0.45) - (BT2020_ALPHA - 1.0)
    };
    enc.copysign(l)
}
