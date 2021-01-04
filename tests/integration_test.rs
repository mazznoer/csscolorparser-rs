extern crate csscolorparser;

use csscolorparser::{parse, Color};

#[test]
fn test_color() {
    let c = Color::from_rgb(1., 0., 0.);
    assert_eq!(c.rgba(), (1., 0., 0., 1.));
    assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    assert_eq!(c.to_hex_string(), "#ff0000");
    assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,1)");
    assert_eq!(c.to_hsva(), (0., 1., 1., 1.));
    assert_eq!(c.to_hsla(), (0., 1., 0.5, 1.));
    assert_eq!(c.to_hwba(), (0., 0., 0., 1.));

    let c = Color::from_rgba(1., 0., 0., 0.5);
    assert_eq!(c.rgba(), (1., 0., 0., 0.5));
    assert_eq!(c.rgba_u8(), (255, 0, 0, 127));
    assert_eq!(c.to_hex_string(), "#ff00007f");
    assert_eq!(c.to_rgb_string(), "rgba(255,0,0,0.5)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,0.5)");

    let c = Color::from_rgb(0., 1., 0.);
    assert_eq!(c.to_hsva(), (120., 1., 1., 1.));
    assert_eq!(c.to_hsla(), (120., 1., 0.5, 1.));
    assert_eq!(c.to_hwba(), (120., 0., 0., 1.));

    let c = Color::from_rgb(0.5, 0.5, 0.5);
    assert_eq!(c.to_hsva(), (0., 0., 0.5, 1.));
    assert_eq!(c.to_hsla(), (0., 0., 0.5, 1.));
    assert_eq!(c.to_hwba(), (0., 0.5, 0.5, 1.));

    let data = vec![
        Color::from_rgb(1., 0., 0.),
        Color::from_rgba(1., 0., 0., 1.),
        Color::from_rgb_u8(255, 0, 0),
        Color::from_rgba_u8(255, 0, 0, 255),
        Color::from_hsv(0., 1., 1.),
        Color::from_hsl(360., 1., 0.5),
        Color::from_hwb(0., 0., 0.),
        Color::from_html("red").unwrap(),
        Color::from_html("#f00").unwrap(),
        Color::from_html("hsv(360,100%,100%)").unwrap(),
    ];
    for c in data {
        assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    }

    let a = Color::from_rgb(1., 1., 1.);
    let b = Color::from_rgb(0., 0., 0.);

    assert_eq!(a.interpolate_rgb(&b, 0.0).rgba_u8(), (255, 255, 255, 255));
    assert_eq!(a.interpolate_rgb(&b, 0.5).rgba_u8(), (127, 127, 127, 255));
    assert_eq!(a.interpolate_rgb(&b, 1.0).rgba_u8(), (0, 0, 0, 255));

    assert_eq!(b.interpolate_rgb(&a, 0.0).rgba_u8(), (0, 0, 0, 255));
    assert_eq!(b.interpolate_rgb(&a, 0.5).rgba_u8(), (127, 127, 127, 255));
    assert_eq!(b.interpolate_rgb(&a, 1.0).rgba_u8(), (255, 255, 255, 255));

    assert_eq!(a.interpolate_lrgb(&b, 0.0).rgba_u8(), (255, 255, 255, 255));
    assert_eq!(a.interpolate_lrgb(&b, 0.5).rgba_u8(), (180, 180, 180, 255));
    assert_eq!(a.interpolate_lrgb(&b, 1.0).rgba_u8(), (0, 0, 0, 255));

    let a = Color::from_rgb(0., 1., 0.);
    let b = Color::from_rgb(0., 0., 1.);

    assert_eq!(a.interpolate_hsv(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(a.interpolate_hsv(&b, 0.5).rgba_u8(), (0, 255, 255, 255));
    assert_eq!(a.interpolate_hsv(&b, 1.0).rgba_u8(), (0, 0, 255, 255));
}

#[test]
fn test_parser() {
    let test_data = vec![
        ("transparent", (0, 0, 0, 0)),
        ("rebeccapurple", (102, 51, 153, 255)),
        ("#ff00ff64", (255, 0, 255, 100)),
        ("rgb(247,179,99)", (247, 179, 99, 255)),
        ("rgb(50% 50% 50%)", (127, 127, 127, 255)),
        ("rgb(247,179,99,0.37)", (247, 179, 99, 94)),
        ("hsl(270 0% 50%)", (127, 127, 127, 255)),
        ("hwb(0 50% 50%)", (127, 127, 127, 255)),
        ("hsv(0 0% 50%)", (127, 127, 127, 255)),
        ("hsv(0 0% 100%)", (255, 255, 255, 255)),
        ("hsv(0 0% 19%)", (48, 48, 48, 255)),
    ];
    for (s, expected) in test_data {
        let a = parse(s).unwrap().rgba_u8();
        let b = s.parse::<Color>().unwrap().rgba_u8();
        let c = Color::from_html(s).unwrap().rgba_u8();
        assert_eq!(expected, a);
        assert_eq!(expected, b);
        assert_eq!(expected, c);
    }
}

#[test]
fn test_named_colors() {
    let test_data = vec![
        ("aliceblue", "#f0f8ff"),
        ("bisque", "#ffe4c4"),
        ("chartreuse", "#7fff00"),
        ("coral", "#ff7f50"),
        ("crimson", "#dc143c"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("gold", "#ffd700"),
        ("hotpink", "#ff69b4"),
        ("indigo", "#4b0082"),
        ("lavender", "#e6e6fa"),
        ("plum", "#dda0dd"),
        ("salmon", "#fa8072"),
        ("skyblue", "#87ceeb"),
        ("tomato", "#ff6347"),
        ("violet", "#ee82ee"),
        ("yellowgreen", "#9acd32"),
    ];
    for (s, hex) in test_data {
        let c = parse(s).unwrap();
        assert_eq!(hex, c.to_hex_string());
    }
}

#[test]
fn test_black() {
    let data = vec![
        "black",
        "#000",
        "#000f",
        "#000000",
        "#000000ff",
        "rgb(0,0,0)",
        "rgb(0% 0% 0%)",
        "rgb(0 0 0 100%)",
        "hsl(270,100%,0%)",
        "hwb(90 0% 100%)",
        "hwb(120deg 0% 100% 100%)",
        "hsv(120 100% 0%)",
    ];
    let black = (0, 0, 0, 255);
    for s in data {
        let c = parse(s).unwrap().rgba_u8();
        assert_eq!(black, c);
    }
}

#[test]
fn test_red() {
    let data = vec![
        "red",
        "#f00",
        "#f00f",
        "#ff0000",
        "#ff0000ff",
        "rgb(255,0,0)",
        "rgb(255 0 0)",
        "rgb(700, -99, 0)", // clamp to 0..255
        "rgb(100% 0% 0%)",
        "rgb(200% -10% -100%)", // clamp to 0%..100%
        "rgb(255 0 0 100%)",
        " RGB ( 255 , 0 , 0 ) ",
        "RGB( 255   0   0 )",
        "hsl(0,100%,50%)",
        "hsl(360 100% 50%)",
        "hwb(0 0% 0%)",
        "hwb(360deg 0% 0% 100%)",
        "hsv(0 100% 100%)",
    ];
    let red = (255, 0, 0, 255);
    for s in data {
        let c = parse(s).unwrap().rgba_u8();
        assert_eq!(red, c);
    }
}

#[test]
fn test_lime() {
    let data = vec![
        "lime",
        "#0f0",
        "#0f0f",
        "#00ff00",
        "#00ff00ff",
        "rgb(0,255,0)",
        "rgb(0% 100% 0%)",
        "rgb(0 255 0 / 100%)",
        "rgba(0,255,0,1)",
        "hsl(120,100%,50%)",
        "hsl(120deg 100% 50%)",
        "hsl(-240 100% 50%)",
        "hsl(-240deg 100% 50%)",
        "hsl(0.3333turn 100% 50%)",
        "hsl(133.333grad 100% 50%)",
        "hsl(2.0944rad 100% 50%)",
        "hsla(120,100%,50%,100%)",
        "hwb(120 0% 0%)",
        "hwb(480deg 0% 0% / 100%)",
        "hsv(120 100% 100%)",
    ];
    let lime = (0, 255, 0, 255);
    for s in data {
        let c = parse(s).unwrap().rgba_u8();
        assert_eq!(lime, c);
    }
}

#[test]
fn test_lime_alpha() {
    let data = vec![
        "#00ff007f",
        "rgb(0,255,0,50%)",
        "rgb(0% 100% 0% / 0.5)",
        "rgba(0%,100%,0%,50%)",
        "hsl(120,100%,50%,0.5)",
        "hsl(120deg 100% 50% / 50%)",
        "hsla(120,100%,50%,0.5)",
        "hwb(120 0% 0% / 50%)",
        "hsv(120 100% 100% / 50%)",
    ];
    let lime_alpha = (0, 255, 0, 127);
    for s in data {
        let c = parse(s).unwrap().rgba_u8();
        assert_eq!(lime_alpha, c);
    }
}

#[test]
fn test_invalid_format() {
    let test_data = vec![
        "",
        "bloodred",
        "#78afzd",
        "#fffff",
        "rgb(255,0,0",
        "rgb(0,255,8s)",
        "rgb(100%,z9%,75%)",
        "cmyk(1 0 0)",
        "rgba(0 0)",
        "hsl(90',100%,50%)",
        "hsl(deg 100% 50%)",
        "hsl(Xturn 100% 50%)",
        "hsl(Zgrad 100% 50%)",
        "hsl(180 1 x%)",
        "hsla(360)",
        "hwb(Xrad,50%,50%)",
        "hwb(270 0% 0% 0% 0%)",
        "hsv(120 100% 100% 1 50%)",
        "hsv(120 XXX 100%)",
    ];
    for s in test_data {
        let c = parse(s);
        assert!(c.is_err());
    }

    let test_data = vec![
        ("#78afzd", "Invalid hex format."),
        ("rgb(255,0)", "Invalid rgb format."),
        ("hsl(360,100%,50%,100%,100%)", "Invalid hsl format."),
        ("hsv(360)", "Invalid hsv format."),
        ("hwb(270,0%,0%,x)", "Invalid hwb format."),
        ("blood", "Invalid unknown format."),
        ("cmyk(0,0,0,0)", "Invalid unknown format."),
    ];
    for (s, err_msg) in test_data {
        let c = parse(s);
        assert_eq!(c.unwrap_err().to_string(), err_msg);
    }
}
