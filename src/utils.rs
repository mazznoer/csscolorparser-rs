mod a98_rgb;
mod calc;
mod color;
mod display_p3;
mod helper;
mod param;
mod prophoto_rgb;
mod rec2020;

pub(crate) use a98_rgb::*;
pub(crate) use calc::*;
pub(crate) use color::*;
pub(crate) use display_p3::*;
pub(crate) use helper::*;
pub(crate) use param::*;
pub(crate) use prophoto_rgb::*;
pub(crate) use rec2020::*;

use core::f32::consts::{PI, TAU};

const PI_3: f32 = PI * 3.0;

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Converts linear RGB components to CIE XYZ (D65 white point).
pub(crate) fn linear_rgb_to_xyz_d65(r: f32, g: f32, b: f32) -> [f32; 3] {
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.119192 * g + 0.9503041 * b;
    [x, y, z]
}

// Converts CIE XYZ (D65 white point) to linear RGB components.
pub(crate) fn xyz_d65_to_linear_rgb(x: f32, y: f32, z: f32) -> [f32; 3] {
    let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g = -0.969266 * x + 1.8760108 * y + 0.0415560 * z;
    let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    [r, g, b]
}

#[allow(clippy::excessive_precision)]
pub(crate) fn oklab_to_linear_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let l_ = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m_ = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s_ = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    let r = 4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_;
    let g = -1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_;
    let b = -0.0041960863 * l_ - 0.7034186147 * m_ + 1.7076147010 * s_;
    [r, g, b]
}

#[allow(clippy::excessive_precision)]
pub(crate) fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let l_ = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m_ = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s_ = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    let l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
    [l, a, b]
}

#[inline]
pub(crate) const fn interp_angle(a0: f32, a1: f32, t: f32) -> f32 {
    let delta = (((a1 - a0) % 360.0) + 540.0) % 360.0 - 180.0;
    (a0 + t * delta + 360.0) % 360.0
}

#[inline]
pub(crate) const fn interp_angle_rad(a0: f32, a1: f32, t: f32) -> f32 {
    let delta = (((a1 - a0) % TAU) + PI_3) % TAU - PI;
    (a0 + t * delta + TAU) % TAU
}

// Map t from range [a, b] to range [c, d]
#[inline]
pub(crate) const fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    (t - a) * ((d - c) / (b - a)) + c
}

#[cfg(test)]
mod t {
    use super::interp_angle;

    #[test]
    fn interp_angle_() {
        let data = [
            ((0.0, 360.0, 0.5), 0.0),
            ((360.0, 90.0, 0.0), 0.0),
            ((360.0, 90.0, 0.5), 45.0),
            ((360.0, 90.0, 1.0), 90.0),
        ];
        for ((a, b, t), expected) in data {
            let v = interp_angle(a, b, t);
            assert_eq!(expected, v);
        }
    }
}
