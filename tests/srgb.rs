use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        ["#ceead2", "color(srgb 0.807 0.919 0.824)"],
        ["#08b44a", "color(srgb 0.031 0.706 0.292)"],
        ["#9db111", "color(srgb 0.614 0.695 0.068)"],
        ["#f2c834", "color(srgb 0.949 0.784 0.203)"],
        ["#d91ca1", "color(srgb 0.851 0.109 0.633)"],
        ["#4f1881", "color(srgb 0.309 0.094 0.506)"],
        ["#cb38fd", "color(srgb 0.798 0.22 0.992)"],
        ["#a3500f", "color(srgb 0.64 0.313 0.057)"],
        ["#25e487", "color(srgb 0.144 0.894 0.529)"],
        ["#63c79c", "color(srgb 0.387 0.781 0.61)"],
        ["#ab0ceb", "color(srgb 0.67 0.049 0.923)"],
        ["#19ac2b", "color(srgb 0.099 0.674 0.169)"],
        ["#ee1da6", "color(srgb 0.933 0.113 0.652)"],
        ["#3ef684", "color(srgb 0.243 0.966 0.519)"],
        ["#0922b9", "color(srgb 0.034 0.134 0.725)"],
        ["#c5f4f2", "color(srgb 0.774 0.955 0.948)"],
        ["#c268f6", "color(srgb 0.762 0.406 0.966)"],
        ["#21384d", "color(srgb 0.128 0.22 0.302)"],
        ["#0671e2", "color(srgb 0.025 0.442 0.887)"],
        ["#85c2f4", "color(srgb 0.523 0.759 0.957)"],
        ["#858e9b", "color(srgb 0.52 0.555 0.608)"],
        // --- With alpha
        ["#3351fa85", "color(srgb 0.2 0.318 0.979 / 52%)"],
        ["#f08177bf", "color(srgb 0.942 0.504 0.466 / 75%)"],
        ["#fdb5861a", "color(srgb 0.994 0.709 0.525 / 10%)"],
        ["#7d5c58ed", "color(srgb 0.491 0.362 0.344 / 93%)"],
        ["#3168c980", "color(srgb 0.193 0.407 0.79 / 50%)"],
        ["#cc3ff196", "color(srgb 0.8 0.248 0.945 / 59%)"],
        ["#9db3e7f7", "color(srgb 0.615 0.702 0.907 / 97%)"],
        ["#0a414a12", "color(srgb 0.038 0.253 0.29 / 7%)"],
        ["#d6f54eab", "color(srgb 0.838 0.962 0.307 / 67%)"],
        ["#1ad90d59", "color(srgb 0.103 0.852 0.052 / 35%)"],
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        let c = c.unwrap();
        assert_eq!(s, c.to_css_color_srgb().to_string(), "{s:?}");
        assert_eq!(hex, c.to_string(), "{s:?}");
    }
}

#[test]
fn relative() {
    let test_data = [
        // none
        ["#000000", "color(from #bad455 srgb none none none)"],
        ["#bad45500", "color(from #bad455 srgb r g b / none)"],
        // bare number
        ["#9b3af4", "color(from #fff srgb 0.608 0.229 0.958)"],
        ["#4a230f", "color(from #fff srgb 0.29 0.137 0.059)"],
        ["#f2f669", "color(from #fff srgb 0.949 0.966 0.412)"],
        ["#a9389c", "color(from #fff srgb 0.664 0.22 0.612)"],
        ["#edc4b6", "color(from #fff srgb 0.928 0.769 0.712)"],
        ["#db778b", "color(from #fff srgb 0.86 0.468 0.547)"],
        ["#259057", "color(from #fff srgb 0.147 0.566 0.34)"],
        ["#1d5e9f", "color(from #fff srgb 0.113 0.367 0.625)"],
        ["#a936c2", "color(from #fff srgb 0.663 0.212 0.759)"],
        ["#22e2ef", "color(from #fff srgb 0.133 0.885 0.939)"],
        ["#3f7353", "color(from #fff srgb 0.248 0.451 0.325)"],
        ["#c4d753", "color(from #fff srgb 0.768 0.844 0.326)"],
        ["#d5a488", "color(from #fff srgb 0.835 0.645 0.532)"],
        ["#d10f04", "color(from #fff srgb 0.819 0.06 0.014)"],
        ["#5445a5", "color(from #fff srgb 0.331 0.272 0.647)"],
        // percentage
        ["#696e3d", "color(from #fff srgb 41% 43% 24%)"],
        ["#572ee0", "color(from #fff srgb 34% 18% 88%)"],
        ["#c2d170", "color(from #fff srgb 76% 82% 44%)"],
        ["#ed29cc", "color(from #fff srgb 93% 16% 80%)"],
        ["#03d9e8", "color(from #fff srgb 1% 85% 91%)"],
        ["#4a4273", "color(from #fff srgb 29% 26% 45%)"],
        ["#bf4773", "color(from #fff srgb 75% 28% 45%)"],
        ["#3be3c4", "color(from #fff srgb 23% 89% 77%)"],
        ["#572121", "color(from #fff srgb 34% 13% 13%)"],
        ["#75bfe3", "color(from #fff srgb 46% 75% 89%)"],
        ["#803b69", "color(from #fff srgb 50% 23% 41%)"],
        ["#4d6bfa", "color(from #fff srgb 30% 42% 98%)"],
        ["#d4d40d", "color(from #fff srgb 83% 83% 5%)"],
        ["#bfa330", "color(from #fff srgb 75% 64% 19%)"],
        ["#2eed54", "color(from #fff srgb 18% 93% 33%)"],
        // keywords
        ["#bad455", "color(from #bad455 srgb r g b)"],
        ["#bad455", "color(from #bad455 srgb r g b / alpha)"],
        // calc(...)
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
