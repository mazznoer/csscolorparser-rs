use csscolorparser::parse;

#[test]
fn one() {
    let test_data = [
        // Absolute
        ["#9ff79a", "color(srgb-linear 0.347 0.93 0.321)"],
        ["#e9fdf0", "color(srgb-linear 0.813 0.982 0.869)"],
        ["#4289d3", "color(srgb-linear 0.054 0.25 0.652)"],
        ["#dcc7c5", "color(srgb-linear 0.716 0.57 0.556)"],
        ["#f3dcbf", "color(srgb-linear 0.898 0.716 0.518)"],
        ["#eed4a4", "color(srgb-linear 0.857 0.659 0.373)"],
        ["#7495e6", "color(srgb-linear 0.176 0.3 0.794)"],
        ["#f0bebb", "color(srgb-linear 0.872 0.517 0.498)"],
        ["#fce56a", "color(srgb-linear 0.974 0.781 0.143)"],
        ["#9a41d1", "color(srgb-linear 0.322 0.053 0.635)"],
        // Absolute (percentage)
        ["#ddc2a8", "color(srgb-linear 72% 54% 39%)"],
        ["#5927bf", "color(srgb-linear 10% 2% 52%)"],
        ["#61e1f0", "color(srgb-linear 12% 75% 87%)"],
        ["#b55d95", "color(srgb-linear 46% 11% 30%)"],
        ["#cad7f7", "color(srgb-linear 59% 68% 93%)"],
        ["#b130f9", "color(srgb-linear 44% 3% 95%)"],
        ["#e6cde8", "color(srgb-linear 79% 61% 81%)"],
        ["#dadad6", "color(srgb-linear 70% 70% 67%)"],
        ["#f8de4b", "color(srgb-linear 94% 73% 7%)"],
        ["#99c759", "color(srgb-linear 32% 57% 10%)"],
        // Relative
        ["#bad455", "color(from #bad455 srgb-linear r g b)"],
        /*[
            "#daa9d56e",
            "color(from #dda0dd srgb-linear calc(r - 0.031) calc(g + 0.052) calc(b * 0.92) / calc(alpha - 0.57))",
        ],*/
    ];
    for [hex, srgb_linear] in test_data {
        let c = parse(srgb_linear).unwrap();
        assert_eq!(hex, c.to_css_hex().to_string(), "{:?}", srgb_linear);
    }
}
