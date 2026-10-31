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
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(srgb none none none / none)"],

        // percentage
        ["#429e8a", "color(srgb 26% 62% 54%)"],
        ["#c9030d", "color(srgb 79% 1% 5%)"],
        ["#474a54", "color(srgb 28% 29% 33%)"],
        ["#380885", "color(srgb 22% 3% 52%)"],
        ["#38ba57", "color(srgb 22% 73% 34%)"],
        ["#5e29b0", "color(srgb 37% 16% 69%)"],
        ["#f53d38", "color(srgb 96% 24% 22%)"],
        ["#8f4ac7", "color(srgb 56% 29% 78%)"],
        ["#40338a", "color(srgb 25% 20% 54%)"],
        ["#faed29", "color(srgb 98% 93% 16%)"],
        ["#7df0f7", "color(srgb 49% 94% 97%)"],
        ["#6e4d3d", "color(srgb 43% 30% 24%)"],
        ["#eb08d6", "color(srgb 92% 3% 84%)"],
        ["#9166a6", "color(srgb 57% 40% 65%)"],
        ["#00b385", "color(srgb 0% 70% 52%)"],
        ["#63ebdb", "color(srgb 39% 92% 86%)"],
        ["#24b526", "color(srgb 14% 71% 15%)"],
        ["#c2f0bd", "color(srgb 76% 94% 74%)"],
        ["#c22eb8", "color(srgb 76% 18% 72%)"],
        ["#b3c9c9", "color(srgb 70% 79% 79%)"],
        ["#829c73", "color(srgb 51% 61% 45%)"],
        ["#6e7a66", "color(srgb 43% 48% 40%)"],
        ["#d9d4a3", "color(srgb 85% 83% 64%)"],
        ["#c23070", "color(srgb 76% 19% 44%)"],
        ["#733608", "color(srgb 45% 21% 3%)"],
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
        ["#d455ba", "color(from #bad455 srgb calc(g) calc(b) calc(r))"],
        ["#bad455", "color(from #fff srgb calc(0.310432 + 8.626015 - 8.207035) calc(16.62746 * 0.5 / (5 * 2)) calc(1.887566 - (1.485768 + 0.068465)))"],
        ["#bad455", "color(from #fff srgb calc(0.081276 + 0.080968 + 2.268672 * 0.25) calc(1.662746 * (0.32124 + 0.17876)) calc((1.133028 + 2.200302) / 10))"],
        ["#bad455", "color(from #fff srgb calc(0.729412 * 2 * (0.269236 + 0.230764)) calc((4.422819 - 1.097327) * 0.25) calc(1.999998 / 2 / (6.738443 - 3.738443)))"],
        ["#bad455", "color(from #fff srgb calc(1.458824 / (20 / 10)) calc(8.31373 / (30 / 3)) calc(1.27777 / 10 + 0.411112 * 0.5))"],
        ["#bad455", "color(from #fff srgb calc(0.048178 + 5.429273 - 4.748039) calc(0.018949 + 0.411918 + 6.945589 - 6.545083) calc(3 * 0.333333 / (4.821452 - 1.821452)))"],
        ["#bad455", "color(from #797d10 srgb calc(7.075108 - (6.820206 - r)) calc(9.639553 + g - 18.596752 / 2) calc(b - (-0.0818688 - 0.1887191)))"],
        ["#bad455", "color(from #24fad5 srgb calc(7.368978 - 6.931341 + 0.432951 - r) calc((g + 2.3451) * 0.025 * 10) calc(6.781558 + b - 7.283519))"],
        ["#bad455", "color(from #40cfe6 srgb calc(0.5 * (r + 1.207844)) calc(g - 0.204771 + 1.121895 / 5) calc(b - 0.568628))"],
        ["#bad455", "color(from #06baab srgb calc((9.0867235 - 8.8367235) * (r + 2.8941186)) calc(g + 0.101961) calc((b + 2.329412) * (5.525898 - 5.414787)))"],
        ["#bad455", "color(from #59526c srgb calc(4 * 0.095098 + r) calc(0.509804 + g) calc(b - 0.90196 / 10))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
