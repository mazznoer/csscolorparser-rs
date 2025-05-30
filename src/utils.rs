#[cfg(feature = "lab")]
use std::f32::consts::{PI, TAU};

#[cfg(feature = "lab")]
const PI_3: f32 = PI * 3.0;

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

pub(crate) fn hue_to_rgb(n1: f32, n2: f32, h: f32) -> f32 {
    let h = modulo(h, 6.0);

    if h < 1.0 {
        return n1 + ((n2 - n1) * h);
    }

    if h < 3.0 {
        return n2;
    }

    if h < 4.0 {
        return n1 + ((n2 - n1) * (4.0 - h));
    }

    n1
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
pub(crate) fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    if s == 0.0 {
        return [l, l, l];
    }

    let n2 = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - (l * s)
    };

    let n1 = 2.0 * l - n2;
    let h = h / 60.0;
    let r = hue_to_rgb(n1, n2, h + 2.0);
    let g = hue_to_rgb(n1, n2, h);
    let b = hue_to_rgb(n1, n2, h - 2.0);
    [r, g, b]
}

pub(crate) fn hwb_to_rgb(hue: f32, white: f32, black: f32) -> [f32; 3] {
    if white + black >= 1.0 {
        let l = white / (white + black);
        return [l, l, l];
    }

    let [r, g, b] = hsl_to_rgb(hue, 1.0, 0.5);
    let r = r * (1.0 - white - black) + white;
    let g = g * (1.0 - white - black) + white;
    let b = b * (1.0 - white - black) + white;
    [r, g, b]
}

#[allow(clippy::float_cmp)]
pub(crate) fn hsv_to_hsl(h: f32, s: f32, v: f32) -> [f32; 3] {
    let l = (2.0 - s) * v / 2.0;

    let s = if l != 0.0 {
        if l == 1.0 {
            0.0
        } else if l < 0.5 {
            s * v / (l * 2.0)
        } else {
            s * v / (2.0 - l * 2.0)
        }
    } else {
        s
    };

    [h, s, l]
}

pub(crate) fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let [h, s, l] = hsv_to_hsl(h, s, v);
    hsl_to_rgb(h, s, l)
}

#[allow(clippy::float_cmp)]
pub(crate) fn rgb_to_hsv(r: f32, g: f32, b: f32) -> [f32; 3] {
    let v = r.max(g.max(b));
    let d = v - r.min(g.min(b));

    if d == 0.0 {
        return [0.0, 0.0, v];
    }

    let s = d / v;
    let dr = (v - r) / d;
    let dg = (v - g) / d;
    let db = (v - b) / d;

    let h = if r == v {
        db - dg
    } else if g == v {
        2.0 + dr - db
    } else {
        4.0 + dg - dr
    };

    let h = (h * 60.0) % 360.0;
    [normalize_angle(h), s, v]
}

#[allow(clippy::float_cmp)]
pub(crate) fn rgb_to_hsl(r: f32, g: f32, b: f32) -> [f32; 3] {
    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let l = (max + min) / 2.0;

    if min == max {
        return [0.0, 0.0, l];
    }

    let d = max - min;

    let s = if l < 0.5 {
        d / (max + min)
    } else {
        d / (2.0 - max - min)
    };

    let dr = (max - r) / d;
    let dg = (max - g) / d;
    let db = (max - b) / d;

    let h = if r == max {
        db - dg
    } else if g == max {
        2.0 + dr - db
    } else {
        4.0 + dg - dr
    };

    let h = (h * 60.0) % 360.0;
    [normalize_angle(h), s, l]
}

pub(crate) fn rgb_to_hwb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let [hue, _, _] = rgb_to_hsl(r, g, b);
    let white = r.min(g.min(b));
    let black = 1.0 - r.max(g.max(b));
    [hue, white, black]
}

#[inline]
pub(crate) fn normalize_angle(t: f32) -> f32 {
    ((t % 360.0) + 360.0) % 360.0
}

#[inline]
pub(crate) fn interp_angle(a0: f32, a1: f32, t: f32) -> f32 {
    let delta = (((a1 - a0) % 360.0) + 540.0) % 360.0 - 180.0;
    (a0 + t * delta + 360.0) % 360.0
}

#[cfg(feature = "lab")]
#[inline]
pub(crate) fn interp_angle_rad(a0: f32, a1: f32, t: f32) -> f32 {
    let delta = (((a1 - a0) % TAU) + PI_3) % TAU - PI;
    (a0 + t * delta + TAU) % TAU
}

#[inline]
pub(crate) fn modulo(x: f32, n: f32) -> f32 {
    (x % n + n) % n
}

// Map t from range [a, b] to range [c, d]
#[inline]
pub(crate) fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    (t - a) * ((d - c) / (b - a)) + c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_angle() {
        let data = vec![
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

    #[test]
    fn test_interp_angle() {
        let data = vec![
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
