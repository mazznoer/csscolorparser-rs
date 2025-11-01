use csscolorparser::parse;

#[test]
fn parser() {
    let test_data = [
        ["rgb(FROM #abcdef g B r / Alpha)", "#cdefab"],
        [
            "rgb(from rgb(from #bad455 g calc(b + 23) r / alpha) b r calc(g - 23))",
            "#bad455",
        ],
        // ---
        ["rgb(from #bad455 r g b)", "#bad455"],
        ["rgb(from #bad455 b r g / alpha)", "#55bad4"],
        ["rgb(from #bad455 255 0 90)", "#ff005a"],
        ["rgb(from #bad455 r g b / 0.2)", "#bad45533"],
        ["rgb(from #bad455 r g b / calc(alpha / 2))", "#bad45580"],
        [
            "rgb(from #bad455 calc(r + 10) calc(g - 15) calc(b * 0.75))",
            "#c4c540",
        ],
        // ---
        ["hwb(from #bad455 h w b)", "#bad455"],
        ["hwb(from #bad455 h b w)", "#90aa2b"],
        ["hwb(from #bad455 0 15 10)", "#e62626"],
        [
            "hwb(from #bad455 calc(h + 90) calc(w - 5) calc(b + 10))",
            "#48bb99",
        ],
        // ---
        ["hsl(from #bad455 h s l)", "#bad455"],
        ["hsl(from #bad455 90 50 65)", "#a6d279"],
        ["hsl(from #bad455 h l s)", "#bbd45c"],
        ["hsl(from #bad455 calc(h - 45) calc(s + 9) l)", "#de8e4b"],
        // ---
        ["oklab(from #bad455 l a b)", "#bad455"],
        ["oklab(from #bad455 l b a)", "#fe9ff5"],
        ["oklab(from #bad455 0.75 -0.2 0.23)", "#60ce00"],
        ["oklab(from #bad455 calc(l * 0.7) a b)", "#708500"],
        // ---
        ["oklch(from #bad455 l c h)", "#bad455"],
        ["oklch(from #bad455 0.75 0.1 170)", "#66c3a4"],
        /*[
            "oklch(from #bad455 calc(l * 1.5) c calc(h + 180))",
            "#ffe7ff",
        ],*/
        [
            "oklch(from #bad455 calc(l - 0.15) calc(c * 0.7) h)",
            "#8fa150",
        ],
    ];
    for [s, hex] in test_data {
        assert_eq!(parse(s).unwrap().to_css_hex(), hex, "{:?}", s);
    }
}

#[cfg(feature = "lab")]
#[test]
fn lab() {
    let test_data = [
        ["lab(from #bad455 l a b)", "#bad455"],
        ["lab(from #bad455 l a b / calc(alpha / 2))", "#bad45580"],
        ["lch(from #bad455 l c h)", "#bad455"],
        ["lch(from #bad455 l c h / calc(alpha * 0.5))", "#bad45580"],
    ];
    for [s, hex] in test_data {
        assert_eq!(parse(s).unwrap().to_css_hex(), hex, "{:?}", s);
    }
}

#[test]
fn invalid() {
    let test_data = [
        "rgb(from)",
        "rgb(from #f00)",
        "rgb(from #f00 r g)",
        "rgb(from #f00 r g b 0.5)",
        "hwb(from #f00 h w b alpha)",
        "rgb(from #f00 r g b / alpha 10)",
        "hsl(from #f00 h s x)",
    ];
    for s in test_data {
        assert!(parse(s).is_err(), "{:?}", s);
    }
}
