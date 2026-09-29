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
        ["#bad455", "rgb(from #bad455 r g b / alpha)"],
        // calc(...)
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
