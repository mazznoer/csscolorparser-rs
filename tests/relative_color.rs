use csscolorparser::parse;

#[test]
fn basic() {
    let hexs = [
        // Solid
        "#ffffff",
        "#000000",
        "#71fe15",
        "#d6e3c9",
        "#2a7719",
        "#b53717",
        "#5b0b8d",
        "#aff632",
        "#65ec8d",
        "#d35493",
        "#289e5f",
        "#b46152",
        "#e0afee",
        "#ac2be4",
        "#233490",
        "#1afbc5",
        "#e41755",
        "#e052ee",
        "#4d1b5e",
        "#230cde",
        "#f8a243",
        "#a130d1",
        "#b38373",
        "",
        // Transparent
        "#6b9fa203",
        "#0e5e0be6",
        "#84f9a716",
        "#48651550",
        "#1adc2cf4",
        "#c191a31c",
        "#a25518c5",
        "#cb33f2c9",
        "#89b21d36",
        "#cbb97f3e",
    ];

    let fmts = [
        "rgb(from {hex} r g b{})",
        "hwb(from {hex} h w b{})",
        "hsl(from {hex} h s l{})",
        "hsv(from {hex} h s v{})",
        "lab(from {hex} l a b{})",
        "lch(from {hex} l c h{})",
        "oklab(from {hex} l a b{})",
        "oklch(from {hex} l c h{})",
        // color(...)
        "color(from {hex} srgb r g b{})",
        "color(from {hex} srgb-linear r g b{})",
        "color(from {hex} xyz-d50 x y z{})",
        "color(from {hex} xyz x y z{})",
        "color(from {hex} xyz-d65 x y z{})",
    ];

    let alphas = [
        "", " / alpha", " / 1",
        //" / 100%",
        //" / 0.5",
    ];

    let mut solid = true;

    for hex in hexs {
        if hex == "" {
            solid = false;
            continue;
        }
        for f in fmts {
            let alphas_ = if solid { &alphas[..] } else { &alphas[0..2] };
            for a in alphas_ {
                // Build string
                let s = f.replace("{hex}", hex);
                let s = s.replace("{}", a);
                // Test
                let c = parse(&s);
                assert!(c.is_ok(), "{:?}", s);
                assert_eq!(hex, c.unwrap().to_string(), "{:?}", s);
            }
        }
    }
}

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
        ["rgb(from #bad455 calc((r + g) / 2) b g)", "#c755d4"],
        [
            "rgb(from #bad455 127 100 calc(((r + g) + b) / 3))",
            "#7f64a1",
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
        // ---
        ["lab(from #bad455 l a b)", "#bad455"],
        ["lab(from #bad455 l a b / calc(alpha / 2))", "#bad45580"],
        // ---
        ["lch(from #bad455 l c h)", "#bad455"],
        ["lch(from #bad455 l c h / calc(alpha * 0.5))", "#bad45580"],
        // ---
        ["color(from #f00 srgb r g b)", "#ff0000"],
        ["color(from #f00 srgb r g b / 0.5)", "#ff000080"],
    ];
    for [s, hex] in test_data {
        assert_eq!(hex, parse(s).unwrap().to_string(), "{:?}", s);
    }
}

#[test]
fn invalid() {
    let test_data = [
        "rgb(from)",
        "rgb(from #f00)",
        "rgb(from #abx 255 0 0)",
        "rgb(from #f00 r g)",
        "rgb(from #f00 r g b 0.5)",
        "hwb(from #f00 h w b alpha)",
        "rgb(from #f00 r g b / alpha 10)",
        "hsl(from #f00 h s x)",
        "rgb(from hwb(from hsv(90 0.5 v) h w b) 0 0 0)",
        // non ascii
        "rgb(ā #f00 r g b)",
        "rgb(from â r g b)",
        "rgb(from #f00 æ g b)",
        "rgb(from #f00 r g b / æ)",
        "rgb(from #f00 r calc(ã+15) b)",
        "rgb(from #f00 calc(1* (r-æ)) g b)",
        "rgb(from #f00 r g b / 1 ã)",
        "rgbà(from #f00 r g b)",
        "æç(from #f00 r g b)",
    ];
    for s in test_data {
        assert!(parse(s).is_err(), "{:?}", s);
    }
}
