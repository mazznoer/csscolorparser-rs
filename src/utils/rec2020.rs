#![allow(clippy::excessive_precision)]

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

use super::{linear_to_srgb, mul3, srgb_to_linear};

// sRGB <-> Rec.2020 (ITU-R BT.2020) conversion.
//
// Inputs and outputs are gamma-encoded r, g, b components, nominal range 0.0..=1.0.
// Both spaces use D65 white, so no chromatic adaptation is needed.
//
// `rec2020_to_srgb` can return values below 0 or above 1 for colours outside
// the sRGB gamut. The result is not clipped, so the conversion stays invertible.

/// Linear sRGB -> linear Rec.2020 (D65).
const SRGB_TO_REC2020: [[f32; 3]; 3] = [
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
const REC2020_TO_SRGB: [[f32; 3]; 3] = [
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

const BT2020_ALPHA: f32 = 1.099_296_826_809_44;
const BT2020_BETA: f32 = 0.018_053_968_510_807;

/// Convert gamma-encoded sRGB to gamma-encoded Rec.2020.
pub(crate) fn srgb_to_rec2020(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [r, g, b] = mul3(
        &SRGB_TO_REC2020,
        [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b)],
    );
    [rec2020_encode(r), rec2020_encode(g), rec2020_encode(b)]
}

/// Convert gamma-encoded Rec.2020 to gamma-encoded sRGB.
pub(crate) fn rec2020_to_srgb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [r, g, b] = mul3(
        &REC2020_TO_SRGB,
        [rec2020_decode(r), rec2020_decode(g), rec2020_decode(b)],
    );
    [linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b)]
}

fn rec2020_decode(v: f32) -> f32 {
    let a = v.abs();
    let lin = if a < 4.5 * BT2020_BETA {
        a / 4.5
    } else {
        ((a + BT2020_ALPHA - 1.0) / BT2020_ALPHA).powf(1.0 / 0.45)
    };
    lin.copysign(v)
}

fn rec2020_encode(l: f32) -> f32 {
    let a = l.abs();
    let enc = if a < BT2020_BETA {
        4.5 * a
    } else {
        BT2020_ALPHA * a.powf(0.45) - (BT2020_ALPHA - 1.0)
    };
    enc.copysign(l)
}
