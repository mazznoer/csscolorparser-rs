/// Wraps angle in degrees into `[0, 360)`. NaN becomes 0.
pub(crate) const fn normalize_angle(h: f32) -> f32 {
    if h.is_nan() {
        return 0.0;
    }
    let h = h % 360.0;
    if h < 0.0 { h + 360.0 } else { h }
}

/// Hue in degrees (`[0, 360)`) of an RGB color, given its max channel and
/// chroma `d = max - min`. `d` must be non-zero.
const fn hue_from_rgb(r: f32, g: f32, b: f32, max_c: f32, d: f32) -> f32 {
    let sector = if max_c == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max_c == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    let h = sector * 60.0;
    // Guard against float rounding producing exactly 360.
    if h >= 360.0 { h - 360.0 } else { h }
}

// HSL

const fn hsl_channel(n: f32, h: f32, l: f32, a: f32) -> f32 {
    let k = (n + h / 30.0) % 12.0;
    l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
}

/// `h` in degrees, `s` and `l` in `0..=1`. Returns `[r, g, b]` in `0..=1`.
pub(crate) const fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    let h = normalize_angle(h);
    let a = s * l.min(1.0 - l);
    [
        hsl_channel(0.0, h, l, a),
        hsl_channel(8.0, h, l, a),
        hsl_channel(4.0, h, l, a),
    ]
}

/// Returns `[h, s, l]` with `h` in degrees (`0..360`), `s` and `l` in `0..=1`.
pub(crate) const fn rgb_to_hsl(r: f32, g: f32, b: f32) -> [f32; 3] {
    let max_c = r.max(g).max(b);
    let min_c = r.min(g).min(b);
    let l = (max_c + min_c) / 2.0;
    let d = max_c - min_c;

    if d == 0.0 {
        return [0.0, 0.0, l];
    }

    let denom = l.min(1.0 - l);
    let s = if denom == 0.0 {
        0.0
    } else {
        (max_c - l) / denom
    };
    [hue_from_rgb(r, g, b, max_c, d), s, l]
}

// HWB

/// `h` in degrees, `w` (whiteness) and `b` (blackness) in `0..=1`.
/// Returns `[r, g, b]` in `0..=1`.
pub(crate) const fn hwb_to_rgb(h: f32, w: f32, b: f32) -> [f32; 3] {
    // If whiteness + blackness >= 1 the result is an achromatic gray.
    if w + b >= 1.0 {
        let gray = w / (w + b);
        return [gray, gray, gray];
    }

    let [r, g, bl] = hsl_to_rgb(h, 1.0, 0.5);
    let f = 1.0 - w - b;
    [r * f + w, g * f + w, bl * f + w]
}

/// Returns `[h, w, b]` with `h` in degrees (`0..360`), `w` and `b` in `0..=1`.
pub(crate) const fn rgb_to_hwb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let max_c = r.max(g).max(b);
    let min_c = r.min(g).min(b);
    let d = max_c - min_c;

    let h = if d == 0.0 {
        0.0
    } else {
        hue_from_rgb(r, g, b, max_c, d)
    };
    [h, min_c, 1.0 - max_c]
}

// HSV

const fn hsv_channel(n: f32, h: f32, s: f32, v: f32) -> f32 {
    let k = (n + h / 60.0) % 6.0;
    v - v * s * k.min(4.0 - k).clamp(0.0, 1.0)
}

/// `h` in degrees, `s` and `v` in `0..=1`. Returns `[r, g, b]` in `0..=1`.
pub(crate) const fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let h = normalize_angle(h);
    [
        hsv_channel(5.0, h, s, v),
        hsv_channel(3.0, h, s, v),
        hsv_channel(1.0, h, s, v),
    ]
}

/// Returns `[h, s, v]` with `h` in degrees (`0..360`), `s` and `v` in `0..=1`.
pub(crate) const fn rgb_to_hsv(r: f32, g: f32, b: f32) -> [f32; 3] {
    let max_c = r.max(g).max(b);
    let min_c = r.min(g).min(b);
    let d = max_c - min_c;

    let h = if d == 0.0 {
        0.0
    } else {
        hue_from_rgb(r, g, b, max_c, d)
    };
    let s = if max_c == 0.0 { 0.0 } else { d / max_c };
    [h, s, max_c]
}

#[cfg(test)]
mod t {
    use super::normalize_angle;

    #[test]
    fn normalize_angle_() {
        let data = [
            (0.0, 0.0),
            (360.0, 0.0),
            (720.0, 0.0),
            (400.0, 40.0),
            (1155.0, 75.0),
            (-360.0, 0.0),
            (-90.0, 270.0),
            (-765.0, 315.0),
        ];
        for (x, expected) in data {
            let c = normalize_angle(x);
            assert_eq!(expected, c);
        }
    }
}
