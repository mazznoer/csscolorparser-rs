use csscolorparser::{parse, Color, NAMED_COLORS};

#[test]
fn named_colors() {
    let skip_list = ["aqua", "cyan", "fuchsia", "magenta"];

    for (&name, &rgb) in NAMED_COLORS.entries() {
        let c = parse(name).unwrap();
        assert_eq!(c.to_rgba8()[0..3], rgb);

        if skip_list.contains(&name) || name.contains("gray") || name.contains("grey") {
            continue;
        }
        assert_eq!(c.name(), Some(name));

        let [r, g, b] = rgb;
        let c = Color::from_rgba8(r, g, b, 255);
        assert_eq!(c.name(), Some(name));
    }

    // Case-insensitive tests

    macro_rules! cmp {
        ($a:expr, $b:expr) => {
            assert_eq!(parse($a).unwrap().to_rgba8(), parse($b).unwrap().to_rgba8());
        };
    }

    cmp!("red", "RED");
    cmp!("red", "Red");
    cmp!("skyblue", "SKYBLUE");
    cmp!("skyblue", "SkyBlue");

    // Hex

    #[rustfmt::skip]
    let test_data = [
        ("aliceblue",   "#f0f8ff"),
        ("bisque",      "#ffe4c4"),
        ("black",       "#000000"),
        ("chartreuse",  "#7fff00"),
        ("coral",       "#ff7f50"),
        ("crimson",     "#dc143c"),
        ("dodgerblue",  "#1e90ff"),
        ("firebrick",   "#b22222"),
        ("gold",        "#ffd700"),
        ("hotpink",     "#ff69b4"),
        ("indigo",      "#4b0082"),
        ("lavender",    "#e6e6fa"),
        ("lime",        "#00ff00"),
        ("plum",        "#dda0dd"),
        ("red",         "#ff0000"),
        ("salmon",      "#fa8072"),
        ("skyblue",     "#87ceeb"),
        ("tomato",      "#ff6347"),
        ("violet",      "#ee82ee"),
        ("yellowgreen", "#9acd32"),
    ];

    for (name, hex) in test_data {
        let c = csscolorparser::parse(name).unwrap();
        assert_eq!(c.to_css_hex(), hex);

        let c = csscolorparser::parse(hex).unwrap();
        assert_eq!(c.name(), Some(name));
    }

    // Colors without names

    let test_data = [
        Color::new(0.7, 0.8, 0.9, 1.0),
        Color::new(1.0, 0.5, 0.0, 1.0),
        Color::from_rgba8(0, 50, 100, 255),
    ];
    for c in test_data {
        assert!(c.name().is_none());
    }
}
