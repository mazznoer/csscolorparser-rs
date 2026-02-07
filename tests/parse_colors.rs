use csscolorparser::parse_colors;

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
            "red, #bad455,ab9",
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
            assert_eq!(c.to_css_hex(), hex);
        }

        assert!(p.next().is_none());
    }
}
