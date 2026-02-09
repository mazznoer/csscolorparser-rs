#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Constants for D65 white point (normalized to Y=1.0)
const D65_X: f32 = 0.95047;
const D65_Y: f32 = 1.0;
const D65_Z: f32 = 1.08883;

const DELTA: f32 = 6.0 / 29.0;
const DELTA2: f32 = DELTA * DELTA;
const DELTA3: f32 = DELTA2 * DELTA;

// Helper function for LAB to XYZ conversion
const fn lab_to_xyz(l: f32, a: f32, b: f32) -> [f32; 3] {
    let fy = (l + 16.0) / 116.0;
    let fx = fy + a / 500.0;
    let fz = fy - b / 200.0;

    const fn lab_f(t: f32) -> f32 {
        if t > DELTA {
            t * t * t
        } else {
            (t - 16.0 / 116.0) * 3.0 * DELTA2
        }
    }

    let x = D65_X * lab_f(fx);
    let y = D65_Y * lab_f(fy);
    let z = D65_Z * lab_f(fz);
    [x, y, z]
}

#[allow(clippy::excessive_precision)]
// Helper function for XYZ to linear RGB conversion
const fn xyz_to_linear_rgb(x: f32, y: f32, z: f32) -> [f32; 3] {
    // sRGB matrix (D65)
    let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    [r, g, b]
}

#[allow(clippy::excessive_precision)]
// Helper function for linear RGB to XYZ conversion
const fn linear_rgb_to_xyz(r: f32, g: f32, b: f32) -> [f32; 3] {
    // Inverse sRGB matrix (D65)
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;
    [x, y, z]
}

// Helper function for XYZ to LAB conversion
fn xyz_to_lab(x: f32, y: f32, z: f32) -> [f32; 3] {
    let lab_f = |t: f32| -> f32 {
        if t > DELTA3 {
            t.cbrt()
        } else {
            (t / (3.0 * DELTA2)) + (4.0 / 29.0)
        }
    };

    let fx = lab_f(x / D65_X);
    let fy = lab_f(y / D65_Y);
    let fz = lab_f(z / D65_Z);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    [l, a, b]
}

// Convert CIELAB (L*a*b*) to linear RGB
// L: [0, 100], a: [-128, 127], b: [-128, 127]
// Returns RGB in [0, 1] range
pub(crate) const fn lab_to_linear_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let [x, y, z] = lab_to_xyz(l, a, b);
    xyz_to_linear_rgb(x, y, z)
}

// Convert linear RGB to CIELAB (L*a*b*)
// RGB components in [0, 1] range
// Returns [L, a, b] with L: [0, 100], a: [-128, 127], b: [-128, 127]
pub(crate) fn linear_rgb_to_lab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [x, y, z] = linear_rgb_to_xyz(r, g, b);
    xyz_to_lab(x, y, z)
}
