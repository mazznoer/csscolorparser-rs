use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(srgb-linear none none none / none)"],

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
    #[rustfmt::skip]
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
        ["#bad455", "color(from #bad455 srgb-linear calc(r) calc(g) calc(b))"],
        ["#bad455", "color(from #fff srgb-linear calc(6.273348 + 2.635799 - (1.765538 + 6.652588)) calc((7.855699 - 4.563824) / (1.680393 + 3.319607)) calc(1.816834 / 5 / 4))"],
        ["#bad455", "color(from #fff srgb-linear calc(12.275525 / 5 / (9.970386 - 4.970386)) calc((7.817321 - 7.685646) * 5 * 1) calc(13.7093106 / 2 - (8.6283233 - 1.8645097)))"],
        ["#bad455", "color(from #fff srgb-linear calc(1.515402 - 1.238052 + 0.854684 * 0.25) calc(2 * 0.125 * 7.9005 / 3) calc(0.0528517 + 0.0131702 + 0.0248198))"],
        ["#bad455", "color(from #fff srgb-linear calc(0.328459 + 0.031842 + 0.13072) calc(0.658375 * 2 * (0.067566 + 0.432434)) calc(0.1816834 * (6.5163563 - 6.0163563)))"],
        ["#bad455", "color(from #fff srgb-linear calc(0.25 * 0.863264 + 1.10082 / 4) calc(0.43638 + 0.221995) calc(1.816834 / 5 / (2.4048269 + 1.5951731)))"],
        ["#bad455", "color(from #cca98b srgb-linear calc(0.751816 * 0.5 - 0.488714 + r) calc(g + 5.213094 - 2.475737 / 0.5) calc(0.908417 / 5 / (2.258183 - b)))"],
        ["#bad455", "color(from #e9548d srgb-linear calc(9.288919 - r - 7.983051) calc(0.7269493 - g + 0.0401626 * 0.5) calc(b + 9.5064288 - (6.282671 + 3.3992721)))"],
        ["#bad455", "color(from #e21460 srgb-linear calc(r - 0.269504) calc(0.13531346 - g + 0.25 * 2.12022788) calc(4.1668669 - b - (3.9393827 + 0.0196715)))"],
        ["#bad455", "color(from #0cd42d srgb-linear calc((5.0880391 - 4.8425286) / (0.5036764 - r)) calc(0.053509 + 2.373046 - (1.109805 + g)) calc((b + 0.4737588) * (1.2185254 - 1.036842)))"],
        ["#bad455", "color(from #dd950c srgb-linear calc(-0.464068 * 0.5 + r) calc(18.402444 * 0.5 - (8.843391 - g)) calc(0.03418705 + 0.05606969 + b / 6.28506223))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
