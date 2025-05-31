use csscolorparser::Color;
use std::convert::TryFrom;

#[test]
fn basic() {
    let c = Color::new(1.0, 0.0, 0.0, 1.0);
    assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(c.to_rgba16(), [65535, 0, 0, 65535]);
    assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(c.to_css_hex(), "#ff0000");
    assert_eq!(c.to_css_rgb(), "rgb(255 0 0)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,1)");
    assert_eq!(c.to_hsva(), [0.0, 1.0, 1.0, 1.0]);
    assert_eq!(c.to_hsla(), [0.0, 1.0, 0.5, 1.0]);
    assert_eq!(c.to_hwba(), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.to_linear_rgba(), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.to_linear_rgba_u8(), [255, 0, 0, 255]);

    let c = Color::new(1.0, 0.0, 0.0, 0.5);
    assert_eq!(c.to_rgba8(), [255, 0, 0, 128]);
    assert_eq!(c.to_css_hex(), "#ff000080");
    assert_eq!(c.to_css_rgb(), "rgb(255 0 0 / 50%)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,0.5)");

    let c = Color::new(0.0, 1.0, 0.0, 1.0);
    assert_eq!(c.to_hsva(), [120.0, 1.0, 1.0, 1.0]);
    assert_eq!(c.to_hsla(), [120.0, 1.0, 0.5, 1.0]);
    assert_eq!(c.to_hwba(), [120.0, 0.0, 0.0, 1.0]);

    let c = Color::new(0.0, 0.0, 1.0, 1.0);
    assert_eq!(c.to_hsva(), [240.0, 1.0, 1.0, 1.0]);
    assert_eq!(c.to_hsla(), [240.0, 1.0, 0.5, 1.0]);
    assert_eq!(c.to_hwba(), [240.0, 0.0, 0.0, 1.0]);

    let c = Color::new(0.0, 0.0, 0.6, 1.0);
    assert_eq!(c.to_hsva(), [240.0, 1.0, 0.6, 1.0]);
    assert_eq!(c.to_hsla(), [240.0, 1.0, 0.3, 1.0]);
    //assert_eq!(c.to_hwba(), [240.0, 0.0, 0.4, 1.0]);

    let c = Color::new(0.5, 0.5, 0.5, 1.0);
    assert_eq!(c.to_hsva(), [0.0, 0.0, 0.5, 1.0]);
    assert_eq!(c.to_hsla(), [0.0, 0.0, 0.5, 1.0]);
    assert_eq!(c.to_hwba(), [0.0, 0.5, 0.5, 1.0]);

    #[cfg(feature = "lab")]
    {
        let c = Color::from_laba(0.0, 0.0, 0.0, 1.0);
        assert_eq!(c.to_rgba8(), [0, 0, 0, 255]);

        let c = Color::from_laba(100.0, 0.0, 0.0, 1.0);
        assert_eq!(c.to_rgba8(), [255, 255, 255, 255]);

        let c = Color::from_lcha(0.0, 0.0, 0.0, 1.0);
        assert_eq!(c.to_rgba8(), [0, 0, 0, 255]);

        let c = Color::from_lcha(100.0, 0.0, 0.0, 1.0);
        assert_eq!(c.to_rgba8(), [255, 255, 255, 255]);
    }

    assert_eq!(Color::default().to_rgba8(), [0, 0, 0, 255]);

    assert_eq!(
        Color::try_from("#f00").unwrap().to_rgba8(),
        [255, 0, 0, 255]
    );

    assert_eq!(
        Color::from((1.0, 0.0, 0.0, 0.5)).to_rgba8(),
        [255, 0, 0, 128]
    );
    assert_eq!(Color::from((1.0, 0.0, 0.0)).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(Color::from((255, 0, 0, 128)).to_rgba8(), [255, 0, 0, 128]);
    assert_eq!(Color::from((255, 0, 0)).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(
        Color::from([1.0, 0.0, 0.0, 0.5]).to_rgba8(),
        [255, 0, 0, 128]
    );
    assert_eq!(Color::from([1.0, 0.0, 0.0]).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(Color::from([255, 0, 0, 128]).to_rgba8(), [255, 0, 0, 128]);
    assert_eq!(Color::from([255, 0, 0]).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(
        Color::from([0.0_f32, 1.0, 0.5, 1.0]).to_rgba8(),
        [0, 255, 128, 255]
    );
    assert_eq!(
        Color::from([0.0_f32, 1.0, 0.5]).to_rgba8(),
        [0, 255, 128, 255]
    );

    // clamping

    let c = Color::new(1.23, 0.5, -0.01, 1.01);
    assert_eq!([c.r, c.g, c.b, c.a], [1.23, 0.5, -0.01, 1.01]);
    assert_eq!(c.to_array(), [1.0, 0.5, 0.0, 1.0]);
    assert_eq!(c.to_rgba8(), [255, 128, 0, 255]);
    assert_eq!(c.to_rgba16(), [65535, 32768, 0, 65535]);

    let c = Color::new(1.23, 0.5, -0.01, 1.01).clamp();
    assert_eq!([c.r, c.g, c.b, c.a], [1.0, 0.5, 0.0, 1.0]);
}

#[test]
fn convert_colors() {
    let colors = &[
        //Color::new(1.0, 0.7, 0.1, 1.0), //
        Color::from_rgba8(255, 179, 26, 255),
        Color::from_rgba8(10, 255, 125, 0),
        Color::from_linear_rgba(0.1, 0.9, 1.0, 1.0),
        Color::from_hwba(0.0, 0.0, 0.0, 1.0),
        Color::from_hwba(320.0, 0.1, 0.3, 1.0),
        Color::from_hsva(120.0, 0.3, 0.2, 0.1),
        Color::from_hsla(120.0, 0.3, 0.2, 1.0),
    ];
    for (i, col) in colors.iter().enumerate() {
        println!("{i} -> {}, {}", &col.to_css_hex(), &col.to_css_rgb());

        let [a, b, c, d] = col.to_linear_rgba();
        let x = Color::from_linear_rgba(a, b, c, d);
        assert_eq!(&col.to_css_hex(), &x.to_css_hex());

        let [a, b, c, d] = col.to_oklaba();
        let x = Color::from_oklaba(a, b, c, d);
        assert_eq!(&col.to_css_hex(), &x.to_css_hex());
    }

    let data = &[
        "#000000",
        "#ffffff",
        "#999999",
        "#7f7f7f",
        "#ff0000",
        "#fa8072",
        "#87ceeb",
        "#ff6347",
        "#ee82ee",
        "#9acd32",
        "#0aff7d",
        "#09ff7d",
        "#ffb31a",
        "#0aff7d",
        "#09ff7d",
        "#825dfa6d",
        "#abc5679b",
    ];
    for s in data {
        let col = csscolorparser::parse(s).unwrap();
        assert_eq!(s, &col.to_css_hex());

        let [a, b, c, d] = col.to_rgba8();
        let x = Color::from_rgba8(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        let [a, b, c, d] = col.to_hsva();
        let x = Color::from_hsva(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        let [a, b, c, d] = col.to_hsla();
        let x = Color::from_hsla(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        let [a, b, c, d] = col.to_hwba();
        let x = Color::from_hwba(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        let [a, b, c, d] = col.to_linear_rgba();
        let x = Color::from_linear_rgba(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        let [a, b, c, d] = col.to_oklaba();
        let x = Color::from_oklaba(a, b, c, d);
        assert_eq!(s, &x.to_css_hex());

        #[cfg(feature = "lab")]
        {
            let [a, b, c, d] = col.to_laba();
            let x = Color::from_laba(a, b, c, d);
            assert_eq!(s, &x.to_css_hex());

            let [a, b, c, d] = col.to_lcha();
            let x = Color::from_lcha(a, b, c, d);
            assert_eq!(s, &x.to_css_hex());
        }
    }
}

#[test]
fn red() {
    let data = &[
        Color::new(1.0, 0.0, 0.0, 1.0),
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_linear_rgba(1.0, 0.0, 0.0, 1.0),
        Color::from_linear_rgba8(255, 0, 0, 255),
        Color::from_hsva(0.0, 1.0, 1.0, 1.0),
        Color::from_hsla(360.0, 1.0, 0.5, 1.0),
        Color::from_hwba(0.0, 0.0, 0.0, 1.0),
        Color::from_oklaba(
            0.6279151939969809,
            0.2249032308661071,
            0.12580287012451802,
            1.0,
        ),
        Color::from_html("#f00").unwrap(),
        Color::from_html("hsv(360,100%,100%)").unwrap(),
    ];
    for c in data {
        assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    }
}

#[test]
fn interpolate() {
    let a = Color::new(0.0, 1.0, 0.0, 1.0);
    let b = Color::new(0.0, 0.0, 1.0, 1.0);

    assert_eq!(a.interpolate_rgb(&b, 0.0).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(a.interpolate_rgb(&b, 0.5).to_rgba8(), [0, 128, 128, 255]);
    assert_eq!(a.interpolate_rgb(&b, 1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(b.interpolate_rgb(&a, 0.0).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(b.interpolate_rgb(&a, 0.5).to_rgba8(), [0, 128, 128, 255]);
    assert_eq!(b.interpolate_rgb(&a, 1.0).to_rgba8(), [0, 255, 0, 255]);

    assert_eq!(
        a.interpolate_linear_rgb(&b, 0.0).to_rgba8(),
        [0, 255, 0, 255]
    );
    assert_eq!(
        a.interpolate_linear_rgb(&b, 0.5).to_rgba8(),
        [0, 188, 188, 255]
    );
    assert_eq!(
        a.interpolate_linear_rgb(&b, 1.0).to_rgba8(),
        [0, 0, 255, 255]
    );

    assert_eq!(a.interpolate_hsv(&b, 0.0).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(a.interpolate_hsv(&b, 0.5).to_rgba8(), [0, 255, 255, 255]);
    assert_eq!(a.interpolate_hsv(&b, 1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(a.interpolate_oklab(&b, 0.0).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(a.interpolate_oklab(&b, 0.5).to_rgba8(), [0, 170, 191, 255]);
    assert_eq!(a.interpolate_oklab(&b, 1.0).to_rgba8(), [0, 0, 255, 255]);

    #[cfg(feature = "lab")]
    {
        assert_eq!(a.interpolate_lab(&b, 0.0).to_rgba8(), [0, 255, 0, 255]);
        assert_eq!(a.interpolate_lab(&b, 1.0).to_rgba8(), [0, 0, 255, 255]);

        assert_eq!(a.interpolate_lch(&b, 0.0).to_rgba8(), [0, 255, 0, 255]);
        assert_eq!(a.interpolate_lch(&b, 1.0).to_rgba8(), [0, 0, 255, 255]);
    }
}
