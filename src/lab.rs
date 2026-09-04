#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

// Standard D50 Illuminant values for CIE LAB (Normalized to Y = 1.0)
const D50_X: f32 = 0.96422;
const D50_Y: f32 = 1.00000;
const D50_Z: f32 = 0.82521;

const DELTA: f32 = 6.0 / 29.0;
const DELTA2: f32 = DELTA * DELTA;
const DELTA3: f32 = DELTA2 * DELTA;

// Helper function for LAB to D50 XYZ conversion
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

    let x = D50_X * lab_f(fx);
    let y = D50_Y * lab_f(fy);
    let z = D50_Z * lab_f(fz);
    [x, y, z]
}

#[allow(clippy::excessive_precision)]
// Convert D50 XYZ to linear sRGB (D65) using CSS Color 4 Bradford Matrix
const fn xyz_to_linear_rgb(x: f32, y: f32, z: f32) -> [f32; 3] {
    let r = 3.1338561 * x - 1.6168667 * y - 0.4906146 * z;
    let g = -0.9787684 * x + 1.9161415 * y + 0.0334540 * z;
    let b = 0.0719453 * x - 0.2289914 * y + 1.4052427 * z;
    [r, g, b]
}

#[allow(clippy::excessive_precision)]
// Convert linear sRGB (D65) to D50 XYZ using CSS Color 4 Bradford Matrix
const fn linear_rgb_to_xyz(r: f32, g: f32, b: f32) -> [f32; 3] {
    let x = 0.4360747 * r + 0.3850649 * g + 0.1430804 * b;
    let y = 0.2225045 * r + 0.7168786 * g + 0.0606169 * b;
    let z = 0.0139322 * r + 0.0971045 * g + 0.7141733 * b;
    [x, y, z]
}

// Helper function for D50 XYZ to LAB conversion
fn xyz_to_lab(x: f32, y: f32, z: f32) -> [f32; 3] {
    let lab_f = |t: f32| -> f32 {
        if t > DELTA3 {
            t.cbrt()
        } else {
            (t / (3.0 * DELTA2)) + (4.0 / 29.0)
        }
    };

    let fx = lab_f(x / D50_X);
    let fy = lab_f(y / D50_Y);
    let fz = lab_f(z / D50_Z);

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
