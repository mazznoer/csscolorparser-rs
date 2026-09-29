use csscolorparser::parse;

#[test]
fn absolute() {
    let test_data = [
        // bare number
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
        ["#5e3fca", "color(srgb-linear 0.111 0.05 0.593)"],
        ["#d2470d", "color(srgb-linear 0.646 0.063 0.004)"],
        ["#64d2fb", "color(srgb-linear 0.128 0.643 0.965)"],
        ["#a1a122", "color(srgb-linear 0.356 0.357 0.016)"],
        ["#93d06b", "color(srgb-linear 0.291 0.628 0.146)"],
        // percentage
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
        ["#a2d3e2", "color(srgb-linear 36% 65% 76%)"],
        ["#38d0af", "color(srgb-linear 4% 63% 43%)"],
        ["#e869c5", "color(srgb-linear 81% 14% 56%)"],
        ["#fdc761", "color(srgb-linear 98% 57% 12%)"],
        ["#00badf", "color(srgb-linear 0% 49% 74%)"],
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}

#[test]
fn relative() {
    let test_data = [
        // none
        ["#000000", "color(from #bad455 srgb-linear none none none)"],
        ["#bad45500", "color(from #bad455 srgb-linear r g b / none)"],
        // bare number
        ["#d8b1d5", "color(from #fff srgb-linear 0.687 0.437 0.665)"],
        ["#73ff6f", "color(from #fff srgb-linear 0.173 0.997 0.159)"],
        ["#86badb", "color(from #fff srgb-linear 0.24 0.491 0.711)"],
        ["#21a2da", "color(from #fff srgb-linear 0.015 0.361 0.704)"],
        ["#f5d3e3", "color(from #fff srgb-linear 0.909 0.65 0.769)"],
        ["#c7e0f5", "color(from #fff srgb-linear 0.571 0.742 0.911)"],
        //["#dcf4f8", "color(from #fff srgb-linear 0.712 0.906 0.935)"],
        ["#a9a3d0", "color(from #fff srgb-linear 0.399 0.364 0.63)"],
        ["#58af31", "color(from #fff srgb-linear 0.097 0.43 0.031)"],
        ["#c7c13a", "color(from #fff srgb-linear 0.57 0.531 0.043)"],
        ["#b1b0c9", "color(from #fff srgb-linear 0.439 0.435 0.585)"],
        ["#e0d3e4", "color(from #fff srgb-linear 0.744 0.653 0.776)"],
        ["#ed6bc1", "color(from #fff srgb-linear 0.844 0.147 0.531)"],
        ["#86daf8", "color(from #fff srgb-linear 0.238 0.7 0.942)"],
        ["#ebb56a", "color(from #fff srgb-linear 0.834 0.463 0.143)"],
        // percentage
        ["#559797", "color(from #fff srgb-linear 9% 31% 31%)"],
        ["#f2edba", "color(from #fff srgb-linear 89% 85% 49%)"],
        ["#005559", "color(from #fff srgb-linear 0% 9% 10%)"],
        ["#847eec", "color(from #fff srgb-linear 23% 21% 84%)"],
        ["#5dc7b1", "color(from #fff srgb-linear 11% 57% 44%)"],
        ["#a64bc8", "color(from #fff srgb-linear 38% 7% 58%)"],
        ["#f5d100", "color(from #fff srgb-linear 91% 64% 0%)"],
        ["#b8acdb", "color(from #fff srgb-linear 48% 41% 71%)"],
        ["#9361f2", "color(from #fff srgb-linear 29% 12% 89%)"],
        ["#ecbda4", "color(from #fff srgb-linear 84% 51% 37%)"],
        ["#dfdd8b", "color(from #fff srgb-linear 74% 72% 26%)"],
        ["#3fcab8", "color(from #fff srgb-linear 5% 59% 48%)"],
        ["#f8a661", "color(from #fff srgb-linear 94% 38% 12%)"],
        ["#bf6f55", "color(from #fff srgb-linear 52% 16% 9%)"],
        ["#818ba8", "color(from #fff srgb-linear 22% 26% 39%)"],
        // keywords
        ["#bad455", "color(from #bad455 srgb-linear r g b)"],
        ["#bad455", "color(from #bad455 srgb-linear r g b / alpha)"],
        // calc(...)
        /*[
            "#daa9d56e",
            "color(from #dda0dd srgb-linear calc(r - 0.031) calc(g + 0.052) calc(b * 0.92) / calc(alpha - 0.57))",
        ],*/
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
