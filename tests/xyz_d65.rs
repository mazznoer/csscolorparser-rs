use csscolorparser::parse;

#[test]
fn absolute() {
    let test_data = [
        // none
        ["#00000000", "color(xyz none none none / none)"],
        // bare number
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
        // percentage
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
        ["#000000", "color(from #bad455 xyz none none none)"],
        ["#bad45500", "color(from #bad455 xyz-d65 x y z / none)"],
        // bare number
        ["#ffbce2", "color(from #fff xyz-d65 0.938 0.734 0.81)"],
        ["#d50037", "color(from #fff xyz-d65 0.279 0.14 0.048)"],
        ["#824323", "color(from #fff xyz-d65 0.115 0.089 0.027)"],
        ["#331b00", "color(from #fff xyz-d65 0.015 0.014 -0.013)"],
        ["#dae600", "color(from #fff xyz-d65 0.569 0.714 0.092)"],
        ["#00ff00", "color(from #fff xyz-d65 0.434 0.904 0.056)"],
        ["#004d00", "color(from #fff xyz-d65 -0.004 0.038 -0.011)"],
        ["#005b40", "color(from #fff xyz-d65 0.006 0.058 0.059)"],
        ["#0090b7", "color(from #fff xyz-d65 0.065 0.172 0.476)"],
        ["#009efb", "color(from #fff xyz-d65 0.152 0.239 0.952)"],
        ["#00ffff", "color(from #fff xyz-d65 0.829 0.913 2.811)"],
        ["#0078ff", "color(from #fff xyz-d65 0.305 0.223 1.567)"],
        ["#ffa6ff", "color(from #fff xyz-d65 1.105 0.717 2.627)"],
        ["#ff00fe", "color(from #fff xyz-d65 0.682 0.252 0.949)"],
        ["#a1878e", "color(from #fff xyz-d65 0.282 0.268 0.294)"],
        // percentage
        ["#9f697a", "color(from #fff xyz-d65 23% 19% 21%)"],
        ["#c3002a", "color(from #fff xyz-d65 22% 10% 3%)"],
        ["#7f3406", "color(from #fff xyz-d65 10% 7% 1%)"],
        ["#eca600", "color(from #fff xyz-d65 48% 45% 5%)"],
        ["#4c5500", "color(from #fff xyz-d65 6% 8% -0%)"],
        ["#006800", "color(from #fff xyz-d65 1% 8% -4%)"],
        ["#00de66", "color(from #fff xyz-d65 22% 50% 21%)"],
        ["#b1ddd4", "color(from #fff xyz-d65 56% 66% 72%)"],
        ["#008eae", "color(from #fff xyz-d65 7% 17% 43%)"],
        ["#00e2ff", "color(from #fff xyz-d65 44% 60% 151%)"],
        ["#708ca7", "color(from #fff xyz-d65 23% 25% 40%)"],
        ["#002d81", "color(from #fff xyz-d65 4% 3% 21%)"],
        ["#5d29af", "color(from #fff xyz-d65 13% 7% 41%)"],
        ["#de91cc", "color(from #fff xyz-d65 51% 40% 62%)"],
        ["#ff00ff", "color(from #fff xyz-d65 187% 89% 106%)"],
        // keywords
        ["#bad455", "color(from #bad455 xyz-d65 x y z)"],
        ["#bad455", "color(from #bad455 xyz x y z / alpha)"],
        // calc(...)
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
