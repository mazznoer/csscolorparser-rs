use csscolorparser::parse;

#[test]
fn one() {
    let test_data = [
        // Absolute
        ["#770024", "color(xyz-d65 0.066 0.013 0.016)"],
        ["#ffdddf", "color(xyz-d65 0.814 0.789 0.804)"],
        ["#f80000", "color(xyz-d65 0.345 0.117 -0.003)"],
        ["#ff5300", "color(xyz-d65 0.798 0.458 0.006)"],
        ["#cb5f00", "color(xyz-d65 0.278 0.205 -0.021)"],
        ["#fffa50", "color(xyz-d65 0.894 0.967 0.216)"],
        ["#bace45", "color(xyz-d65 0.434 0.549 0.139)"],
        ["#e3ffc3", "color(xyz-d65 0.795 0.963 0.658)"],
        ["#abbfac", "color(xyz-d65 0.428 0.488 0.462)"],
        ["#00964b", "color(xyz-d65 0.073 0.197 0.1)"],
        ["#00b79d", "color(xyz-d65 0.21 0.354 0.376)"],
        ["#00ffff", "color(xyz-d65 0.433 0.836 1.378)"],
        ["#00ade3", "color(xyz-d65 0.157 0.288 0.772)"],
        ["#00d0ff", "color(xyz-d65 0.372 0.501 1.313)"],
        ["#0051dc", "color(xyz-d65 0.055 0.057 0.686)"],
        ["#7aaeff", "color(xyz-d65 0.413 0.417 1.008)"],
        ["#0048ff", "color(xyz-d65 0.16 0.096 0.966)"],
        ["#4b4154", "color(xyz-d65 0.064 0.059 0.092)"],
        ["#e593eb", "color(xyz-d65 0.578 0.435 0.839)"],
        //["#f2d5e6", "color(xyz-d65 0.747 0.72 0.846)"],
        ["#ffdfff", "color(xyz-d65 1.208 0.992 1.104)"],
        // Absolute (percentage)
        ["#ff65b6", "color(xyz 72% 43% 49%)"],
        ["#ff289f", "color(xyz 135% 70% 39%)"],
        ["#df9886", "color(xyz 46% 40% 28%)"],
        ["#d0661d", "color(xyz 31% 23% 4%)"],
        ["#5b4828", "color(xyz 7% 7% 3%)"],
        ["#c0a500", "color(xyz 35% 38% 4%)"],
        ["#779800", "color(xyz 18% 26% 0%)"],
        ["#c0ff83", "color(xyz 66% 93% 36%)"],
        ["#69c077", "color(xyz 28% 42% 24%)"],
        ["#00e999", "color(xyz 32% 59% 40%)"],
        ["#004e42", "color(xyz 2% 5% 6%)"],
        ["#008091", "color(xyz 4% 13% 29%)"],
        ["#005179", "color(xyz 2% 5% 19%)"],
        ["#202638", "color(xyz 2% 2% 4%)"],
        ["#004e94", "color(xyz 5% 6% 29%)"],
        ["#00ffff", "color(xyz 105% 102% 375%)"],
        ["#909fff", "color(xyz 47% 40% 126%)"],
        ["#771dff", "color(xyz 31% 14% 121%)"],
        ["#312f37", "color(xyz 3% 3% 4%)"],
        // Relative
        ["#bad455", "color(from #bad455 xyz-d65 x y z)"],
        ["#bad455", "color(from #bad455 xyz x y z / alpha)"],
    ];
    for [hex, xyz_d65] in test_data {
        let c = parse(xyz_d65).unwrap();
        assert_eq!(hex, c.to_css_hex().to_string(), "{:?}", xyz_d65);
    }
}
