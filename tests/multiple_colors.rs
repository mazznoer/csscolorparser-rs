use csscolorparser::{ParseColorError, parse_colors};

#[test]
fn basic() {
    #[rustfmt::skip]
    let test_data = [
        "",
        " ",
        "  \t \n ",
        ", ,, ",
    ];

    for s in test_data {
        let mut p = parse_colors(s);
        assert!(p.next().is_none());
    }

    #[rustfmt::skip]
    let test_data = [
        (
            "a3f",
            vec!["#aa33ff"],
        ),
        (
            ",red, #bad455,ab9,",
            vec!["#ff0000", "#bad455", "#aabb99"],
        ),
        (
            "rgb(0,255,0),#abc,hsl(0, 100%, 50%), , hwb(0 0% 0%) ",
            vec!["#00ff00", "#aabbcc", "#ff0000", "#ff0000"],
        ),
        (
            "#f00, rgb(from hwb(from #bad455 calc(h * (1+0)) w b) r g b), #123abc",
            vec!["#ff0000", "#bad455", "#123abc"],
        ),
        (
            "red, #0ff, âßï, rgb(0,0,255)",
            vec!["#ff0000", "#00ffff", "", "#0000ff"],
        ),
        (
            "#0f0,\t#00f\n,\r #ff0",
            vec!["#00ff00", "#0000ff", "#ffff00"],
        ),
    ];

    for (s, result) in test_data {
        let mut p = parse_colors(s);

        for hex in result {
            let c = p.next();
            assert!(c.is_some());
            let c = c.unwrap();
            if hex == "" {
                assert!(c.is_err());
                continue;
            }
            assert!(c.is_ok());
            let c = c.unwrap();
            assert_eq!(c.to_css_hex().to_string(), hex);
        }

        assert!(p.next().is_none());
    }
}

#[test]
fn invalid_colors() {
    fn ps(s: &str) -> String {
        let res: Result<Vec<_>, _> = parse_colors(s).collect();
        res.unwrap_err().to_string()
    }

    #[rustfmt::skip]
    let invalid = [
        [
            "rgb(0,0)",
            "invalid rgb format: \"rgb(0,0)\"",
        ],
        [
            "rgb(0 0 0),#ffx",
            "invalid hex format: \"#ffx\"",
        ],
        [
            "#0f9, hwb(95 0.3 0.7),§ü¥,#f00",
            "invalid unknown format: \"§ü¥\"",
        ],
        [
            "π, #ff0",
            "invalid unknown format: \"π\"",
        ],
    ];

    for [s, err] in invalid {
        assert_eq!(ps(s), err, "{:?}", s);
    }

    // ---

    #[rustfmt::skip]
    let test_data = [
        (
            "rgb(0, 255, 0), #ff000x, #00f",
            " #ff000x",
            ParseColorError::InvalidHex,
        ),
        (
            "rgb(0, 255, 0), rgb(0, 0) , #00f",
            " rgb(0, 0) ",
            ParseColorError::InvalidRgb,
        ),
        (
            "#0f9, hwb(95 0.3 0.7),§ü¥,#f00",
            "§ü¥",
            ParseColorError::InvalidUnknown,
        ),
        (
            "π, #ff0",
            "π",
            ParseColorError::InvalidUnknown,
        ),
    ];

    for (input, s, err) in test_data {
        let res: Result<Vec<_>, _> = parse_colors(input).collect();
        let e = res.unwrap_err();
        assert_eq!(e.source, err);
        assert_eq!(e.color, s);
    }

    // ---

    let invalid2 = [
        "rgb(from rgb(from rgb(255, 0, 0) r g b), #0f0",
        "rgb(from rgb(from rgb(255,0,0) r g b))), #0f0",
        "rgb((255, 0, 0), #0f0",
    ];

    for s in invalid2 {
        let mut p = parse_colors(s);

        let item = p.next();
        assert!(item.is_some());
        let item = item.unwrap();
        assert!(item.is_err());

        let item = p.next();
        assert!(item.is_some());
        let item = item.unwrap().unwrap();
        assert_eq!(item.to_rgba8(), [0, 255, 0, 255]);

        assert!(p.next().is_none());
    }
}
