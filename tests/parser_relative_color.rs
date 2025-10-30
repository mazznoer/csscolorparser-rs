use csscolorparser::parse;

#[test]
fn parser() {
    let test_data = [
        ("rgb(from #f00 r g b)", "#ff0000"),
        ("rgb(from #f00 r 127 b)", "#ff7f00"),
        ("rgb(FROM #abcdef g B r / Alpha)", "#cdefab"),
        ("rgb(from #00f r calc(g + 90) calc(b / 2))", "#005a80"),
        (
            "rgb(from rgb(from #bad455 g calc(b + 23) r / alpha) b r calc(g - 23))",
            "#bad455",
        ),
    ];
    for (s, hex) in test_data {
        assert_eq!(parse(s).unwrap().to_css_hex(), hex);
    }
}
