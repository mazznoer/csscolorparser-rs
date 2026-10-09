use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        ["#ad115d", "lch(38.46 61.14 0)"],
        ["#2e191a", "lch(11.69 11.88 17.95)"],
        ["#af0000", "lch(30.68 97.2 35.9)"],
        ["#ff914f", "lch(71.59 65.69 53.85)"],
        ["#392000", "lch(14.96 29.96 71.8)"],
        ["#eac800", "lch(81.62 95.83 89.75)"],
        ["#a0db00", "lch(80.75 131.77 107.7)"],
        ["#009700", "lch(53.95 98.36 125.65)"],
        ["#286835", "lch(39.08 38.24 143.6)"],
        ["#004f12", "lch(25.34 74.61 161.55)"],
        ["#00785a", "lch(39.6 85.74 179.5)"],
        ["#00363a", "lch(17.38 37.39 197.45)"],
        ["#004153", "lch(22.44 35.1 215.4)"],
        ["#00284f", "lch(11.01 44.71 233.35)"],
        ["#40617b", "lch(39.37 19.77 251.3)"],
        ["#aee7ff", "lch(90.52 51.04 269.25)"],
        ["#033ca4", "lch(28.24 64.39 287.2)"],
        ["#d7b7fe", "lch(79.12 38.01 305.15)"],
        ["#ab55b3", "lch(49.94 57.7 323.1)"],
        ["#ff00ff", "lch(72.03 148.23 341.05)"],
        ["#ff00a7", "lch(63.48 148.37 359)"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        let c = c.unwrap();
        assert_eq!(s, c.to_css_lch().to_string(), "{s:?}");
        assert_eq!(hex, c.to_string(), "{s:?}");
    }
}

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "lch(none none none / none)"],

        // percentage
        ["#ff0022", "lch(50% 84% 28)"],
        ["#3d006d", "lch(0% 78% 321)"],
        ["#005bbb", "lch(30% 58% 237)"],
        ["#003e9d", "lch(25% 42% 273)"],
        ["#00ffff", "lch(96% 41% 237)"],
        ["#ff58ff", "lch(88% 76% 345)"],
        ["#beb95e", "lch(74% 31% 99)"],
        ["#005e9f", "lch(29% 69% 214)"],
        ["#00d2ff", "lch(81% 71% 269)"],
        ["#00db00", "lch(75% 93% 128)"],

        // angle suffix
        ["#d200a5", "lch(44.83 91.15 376grad)"],
        ["#7cc2a1", "lch(73.18 30.1 2.831rad)"],
        ["#8e0000", "lch(25.03 90.57 0.125turn)"],
        ["#002f25", "lch(9.58 70.34 208grad)"],
        ["#b2501f", "lch(46.59 60.86 0.139turn)"],
        ["#ff00d3", "lch(92.48 129.61 8grad)"],
        ["#00a5cd", "lch(61.23 45.46 251grad)"],
        ["#5f4a39", "lch(33.42 15.49 0.171turn)"],
        ["#f6f933", "lch(95.49 86.25 101deg)"],
        ["#f5b7c7", "lch(80.55 25.16 0.037rad)"],
        ["#0076c7", "lch(44.02 61.45 0.688turn)"],
        ["#becc39", "lch(78.99 68.73 1.833rad)"],
        ["#3d182a", "lch(14.54 20.69 6.115rad)"],
        ["#ffdf00", "lch(91.92 104.44 85deg)"],
        ["#8b0083", "lch(20.55 108.11 5.772rad)"],
        ["#00aba6", "lch(58.37 80.23 3.338rad)"],
        ["#00caff", "lch(72.12 141.3 288grad)"],
        ["#ffd3ff", "lch(99.97 94.49 345grad)"],
        ["#00aeff", "lch(58.91 99.04 248grad)"],
        ["#8a8376", "lch(55.02 7.84 1.432rad)"],
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
        ["#000000", "lch(from #bad455 none none none)"],
        ["#bad45500", "lch(from #bad455 l c h / none)"],

        // bare number
        ["#d98ca4", "lch(from #fff 66.84 32.87 0)"],
        ["#ff002b", "lch(from #fff 52.63 134.14 25.64)"],
        ["#ff730a", "lch(from #fff 70.52 100.02 51.29)"],
        ["#a85200", "lch(from #fff 44.88 137.89 76.93)"],
        ["#d3e900", "lch(from #fff 88.21 116.97 102.57)"],
        ["#c3eda7", "lch(from #fff 89.39 37.24 128.21)"],
        ["#00b757", "lch(from #fff 63.81 79 153.86)"],
        ["#00ffe4", "lch(from #fff 91.84 65.52 179.5)"],
        ["#00738e", "lch(from #fff 38.69 75.73 205.14)"],
        ["#00b1ff", "lch(from #fff 57.34 138.64 230.79)"],
        ["#d6e2f0", "lch(from #fff 89.33 8.64 256.43)"],
        ["#282a36", "lch(from #fff 17.28 8.4 282.07)"],
        ["#ca94ff", "lch(from #fff 69.8 58.41 307.71)"],
        ["#d577c2", "lch(from #fff 62.73 50.6 333.36)"],
        ["#ffc7ea", "lch(from #fff 91.48 42.1 359)"],

        // percentage
        ["#00caff", "lch(from #fff 70% 53% 217)"],
        ["#ffe900", "lch(from #fff 98% 75% 80)"],
        ["#595959", "lch(from #fff 38% 0% 6)"],
        ["#ffa500", "lch(from #fff 77% 65% 71)"],
        ["#f04800", "lch(from #fff 56% 72% 54)"],
        ["#9aecff", "lch(from #fff 92% 44% 270)"],
        ["#779d00", "lch(from #fff 60% 67% 107)"],
        ["#aeaeae", "lch(from #fff 71% 0% 173)"],
        ["#008192", "lch(from #fff 47% 30% 208)"],
        ["#b9754c", "lch(from #fff 56% 28% 54)"],
        ["#700000", "lch(from #fff 22% 47% 52)"],
        ["#ff8dff", "lch(from #fff 91% 63% 344)"],
        ["#426303", "lch(from #fff 38% 32% 118)"],
        ["#00a1ff", "lch(from #fff 63% 45% 264)"],
        ["#fcf5f3", "lch(from #fff 97% 2% 42)"],

        // keywords
        ["#bad455", "lch(from #bad455 l c h)"],
        ["#bad455", "lch(from #bad455 l c h / alpha)"],

        // calc(...)
        ["#bad455", "lch(from #bad455 calc(l) calc(c) calc(h))"],
        ["#00a7ff", "lch(from #bad455 calc(10 + 300 / 6) calc(150 - 0.5 * 90) calc(36 * 7 + 1))"],
        ["#5dc275", "lch(from #bad455 calc(l - 10) calc(c * 0.85) calc(h + 35))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}

#[test]
fn invalid() {
    let test_data = [
        // invalid hue (percentage)
        "lch(0 0 0%)",
        "lch(from #bad455 l c 0%)",
    ];

    for s in test_data {
        assert!(parse(s).is_err(), "{s:?}");
    }
}
