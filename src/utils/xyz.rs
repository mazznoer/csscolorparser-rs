// Convert linear sRGB (D65) to CIE XYZ (D50)
pub(crate) const fn linear_rgb_to_xyz_d50(r: f32, g: f32, b: f32) -> [f32; 3] {
    let x = 0.4360747 * r + 0.3850649 * g + 0.1430804 * b;
    let y = 0.2225045 * r + 0.7168786 * g + 0.0606169 * b;
    let z = 0.0139322 * r + 0.0971045 * g + 0.7141733 * b;
    [x, y, z]
}

// Convert CIE XYZ (D50) to linear sRGB (D65)
pub(crate) const fn xyz_d50_to_linear_rgb(x: f32, y: f32, z: f32) -> [f32; 3] {
    let r = 3.1338561 * x - 1.6168667 * y - 0.4906146 * z;
    let g = -0.9787684 * x + 1.9161415 * y + 0.0334540 * z;
    let b = 0.0719453 * x - 0.2289914 * y + 1.4052427 * z;
    [r, g, b]
}

// Convert linear sRGB (D65) to CIE XYZ (D65)
pub(crate) const fn linear_rgb_to_xyz_d65(r: f32, g: f32, b: f32) -> [f32; 3] {
    let x = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = 0.0193339 * r + 0.119192 * g + 0.9503041 * b;
    [x, y, z]
}

// Convert CIE XYZ (D65) to linear SRGB (D65).
pub(crate) const fn xyz_d65_to_linear_rgb(x: f32, y: f32, z: f32) -> [f32; 3] {
    let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g = -0.969266 * x + 1.8760108 * y + 0.0415560 * z;
    let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    [r, g, b]
}
