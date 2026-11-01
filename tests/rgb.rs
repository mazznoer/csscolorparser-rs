use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        "rgb(71 175 99)",
        "rgb(170 203 72)",
        "rgb(45 232 237)",
        "rgb(119 1 124)",
        "rgb(243 93 86)",
        "rgb(223 25 119)",
        "rgb(6 44 133)",
        "rgb(167 240 237)",
        "rgb(97 71 129)",
        "rgb(125 68 93)",
        "rgb(139 187 62)",
        "rgb(100 51 80)",
        "rgb(27 249 123)",
        "rgb(230 63 99)",
        "rgb(241 34 4)",
        "rgb(149 222 185)",
        "rgb(3 129 213)",
        "rgb(88 220 108)",
        "rgb(199 169 6)",
        "rgb(54 70 163)",
        "rgb(90 42 106)",
    ];

    for s in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(s, c.unwrap().to_css_rgb().to_string(), "{s:?}");
    }
}

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#000000", "rgb(none none none)"],
        ["#ff00ff00", "rgb(255 0 255 / none)"],

        // percentage
        ["#d90d12", "rgb(85% 5% 7%)"],
        ["#b8c729", "rgb(72% 78% 16%)"],
        ["#d16ed9", "rgb(82% 43% 85%)"],
        ["#36add9", "rgb(21% 68% 85%)"],
        ["#9e0f3b", "rgb(62% 6% 23%)"],
        ["#dede3d", "rgb(87% 87% 24%)"],
        ["#03def5", "rgb(1% 87% 96%)"],
        ["#6e594f", "rgb(43% 35% 31%)"],
        ["#2b9914", "rgb(17% 60% 8%)"],
        ["#91b3ed", "rgb(57% 70% 93%)"],
        ["#36171c", "rgb(21% 9% 11%)"],
        ["#e02687", "rgb(88% 15% 53%)"],
        ["#82b89e", "rgb(51% 72% 62%)"],
        ["#7a0f57", "rgb(48% 6% 34%)"],
        ["#1a408f", "rgb(10% 25% 56%)"],
        ["#3b33eb", "rgb(23% 20% 92%)"],
        ["#382682", "rgb(22% 15% 51%)"],
        ["#fc2124", "rgb(99% 13% 14%)"],
        ["#0387bd", "rgb(1% 53% 74%)"],
        ["#308c2e", "rgb(19% 55% 18%)"],
        ["#a68508", "rgb(65% 52% 3%)"],
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
        ["#000000", "rgb(from #fff none none none)"],
        ["#bad45500", "rgb(from #bad455 r g b / none)"],

        // bare number
        ["#df5d70", "rgb(from #fff 223 93 112)"],
        ["#8d4b7b", "rgb(from #fff 141 75 123)"],
        ["#b9615f", "rgb(from #fff 185 97 95)"],
        ["#afc7bf", "rgb(from #fff 175 199 191)"],
        ["#9b6bcf", "rgb(from #fff 155 107 207)"],
        ["#1dc57b", "rgb(from #fff 29 197 123)"],
        ["#02349a", "rgb(from #fff 2 52 154)"],
        ["#a83caf", "rgb(from #fff 168 60 175)"],
        ["#9c77f0", "rgb(from #fff 156 119 240)"],
        ["#9e533a", "rgb(from #fff 158 83 58)"],
        ["#97eb78", "rgb(from #fff 151 235 120)"],
        ["#c8a9da", "rgb(from #fff 200 169 218)"],
        ["#6b6021", "rgb(from #fff 107 96 33)"],
        ["#9accfc", "rgb(from #fff 154 204 252)"],
        ["#a2dd3f", "rgb(from #fff 162 221 63)"],
        ["#34a783", "rgb(from #fff 52 167 131)"],

        // percentage
        ["#e85447", "rgb(from #fff 91% 33% 28%)"],
        ["#63b8fa", "rgb(from #fff 39% 72% 98%)"],
        ["#d6e09e", "rgb(from #fff 84% 88% 62%)"],
        ["#d4d445", "rgb(from #fff 83% 83% 27%)"],
        ["#0fa340", "rgb(from #fff 6% 64% 25%)"],
        ["#5c7d42", "rgb(from #fff 36% 49% 26%)"],
        ["#00cf69", "rgb(from #fff 0% 81% 41%)"],
        ["#17e3d4", "rgb(from #fff 9% 89% 83%)"],
        ["#5e6682", "rgb(from #fff 37% 40% 51%)"],
        ["#ccad69", "rgb(from #fff 80% 68% 41%)"],
        ["#c75221", "rgb(from #fff 78% 32% 13%)"],
        ["#e3f01a", "rgb(from #fff 89% 94% 10%)"],
        ["#52385c", "rgb(from #fff 32% 22% 36%)"],
        ["#9cad7d", "rgb(from #fff 61% 68% 49%)"],
        ["#c229ba", "rgb(from #fff 76% 16% 73%)"],
        ["#0fe8d9", "rgb(from #fff 6% 91% 85%)"],

        // keywords
        ["#bad455", "rgb(from #bad455 r g b)"],
        ["#55bad4", "rgb(from #bad455 b r g / alpha)"],

        // calc(...)
        ["#bad455", "rgb(from #bad455 calc(r) calc(g) calc(b))"],
        ["#a18ac5", "rgb(from #bad455 calc((r + g + b) / 3) calc(12 * 9 + 30) calc(0.197 * 1000))"],
        ["#bad455", "rgb(from #fff calc(97.0503 - 22.27043 + 222.44038 * 0.5) calc((1081.826285 - 21.82571) / 5) calc(428.031666 / 3 - 57.677307))"],
        ["#bad455", "rgb(from #fff calc(18600.006 / 10 / 10) calc(2120.00115 / (2 * 5)) calc(10.154295 + 299.38248 / 4))"],
        ["#bad455", "rgb(from #fff calc(277.64082 - 91.64076) calc(108.961923 + 95.113882 + 7.92431) calc((4.30072 - 1.30072) * 28.333305))"],
        ["#bad455", "rgb(from #fff calc((3010.68724 - 1150.68664) / 10) calc((1400.826705 + 719.174445) / (2 * 5)) calc((32.991384 + 137.008446) / (4.787798 - 2.787798)))"],
        ["#bad455", "rgb(from #fff calc(39.42094 - 18.1707 + 164.74982) calc(848.00046 * 0.25) calc(424.999575 / (50 / 10)))"],
        ["#bad455", "rgb(from #528a63 calc(519.999825 / 5 + r) calc((50.843152 - 8.443129) * (g - 132.99988)) calc(b + 19.394302 - (20.958819 + 12.435493)))"],
        ["#bad455", "rgb(from #5f636a calc(r + 56.179287 + 34.571654 + 0.249124) calc(g + 226.00038 * 0.5) calc((955.99908 - b) / 10))"],
        ["#bad455", "rgb(from #9b2f21 calc((-61.999935 + r) / (0.405516 + 0.094484)) calc(84.800046 / 2 * g / 9.400014) calc(119.727067 + b - 270.908848 / 4))"],
        ["#bad455", "rgb(from #4ee496 calc(108.00015 + r) calc(307.318688 - 121.489763 + g - 201.8289) calc(0.5 * 223.43599 - (176.718005 - b)))"],
        ["#bad455", "rgb(from #2813be calc((102.000085 - r) * (4.035506 - 1.035506)) calc(g + 64.333372 * 3) calc(64599.932 / b * 5 * 0.05))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
