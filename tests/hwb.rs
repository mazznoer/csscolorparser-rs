use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        "hwb(0 87% 0%)",
        "hwb(17 0% 23%)",
        "hwb(35 0% 7%)",
        "hwb(53 66% 0%)",
        "hwb(71 0% 66%)",
        "hwb(89 22% 0%)",
        "hwb(107 0% 2%)",
        "hwb(125 51% 0%)",
        "hwb(143 10% 0%)",
        "hwb(161 0% 76%)",
        "hwb(179 72% 0%)",
        "hwb(197 0% 60%)",
        "hwb(215 0% 39%)",
        "hwb(233 0% 18%)",
        "hwb(251 0% 3%)",
        "hwb(269 57% 0%)",
        "hwb(287 21% 0%)",
        "hwb(305 15% 0%)",
        "hwb(323 55% 0%)",
        "hwb(341 0% 72%)",
        "hwb(359 0% 2%)",
    ];
    for s in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(s, c.unwrap().to_css_hwb().to_string(), "{s:?}");
    }
}

#[test]
fn absolute() {
    let test_data = [
        // none
        ["#ff0000", "hwb(none none none)"],
        ["#00b7ff00", "hwb(197 0 0 / none)"],
        // bare number
        ["#420000", "hwb(0 0 74)"],
        ["#692800", "hwb(23 0 59)"],
        ["#ffdd61", "hwb(47 38 0)"],
        ["#89a800", "hwb(71 0 34)"],
        ["#9bff54", "hwb(95 33 0)"],
        ["#03a300", "hwb(119 0 36)"],
        ["#00b846", "hwb(143 0 28)"],
        ["#003d30", "hwb(167 0 76)"],
        ["#baf2ff", "hwb(191 73 0)"],
        ["#78b0ff", "hwb(215 47 0)"],
        ["#babbff", "hwb(239 73 0)"],
        ["#4300b0", "hwb(263 0 31)"],
        ["#5e0078", "hwb(287 0 53)"],
        ["#ff1ad5", "hwb(311 10 0)"],
        ["#260010", "hwb(335 0 85)"],
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
        ["#ff0000", "hwb(from #bad455 none none none)"],
        ["#bad45500", "hwb(from #bad455 h w b / none)"],
        // bare number
        ["#ff6161", "hwb(from #fff 0 38 0)"],
        ["#ff873d", "hwb(from #fff 23 24 0)"],
        ["#ffedab", "hwb(from #fff 47 67 0)"],
        ["#5e7300", "hwb(from #fff 71 0 55)"],
        ["#aaff6e", "hwb(from #fff 95 43 0)"],
        ["#013d00", "hwb(from #fff 119 0 76)"],
        ["#00b344", "hwb(from #fff 143 0 30)"],
        ["#94ffe8", "hwb(from #fff 167 58 0)"],
        ["#004f61", "hwb(from #fff 191 0 62)"],
        ["#cce1ff", "hwb(from #fff 215 80 0)"],
        ["#6b6eff", "hwb(from #fff 239 42 0)"],
        ["#e9dbff", "hwb(from #fff 263 86 0)"],
        ["#70008f", "hwb(from #fff 287 0 44)"],
        ["#ff85e9", "hwb(from #fff 311 52 0)"],
        ["#e0005d", "hwb(from #fff 335 0 12)"],
        // percentage
        ["#ffa1a1", "hwb(from #fff 0 63% 0%)"],
        ["#c74c00", "hwb(from #fff 23 0% 22%)"],
        ["#4d3c00", "hwb(from #fff 47 0% 70%)"],
        ["#98ba00", "hwb(from #fff 71 0% 27%)"],
        ["#1c4200", "hwb(from #fff 95 0% 74%)"],
        ["#9bff99", "hwb(from #fff 119 60% 0%)"],
        ["#d1ffe3", "hwb(from #fff 143 82% 0%)"],
        ["#005744", "hwb(from #fff 167 0% 66%)"],
        ["#00b1d9", "hwb(from #fff 191 0% 15%)"],
        ["#87b9ff", "hwb(from #fff 215 53% 0%)"],
        ["#00014f", "hwb(from #fff 239 0% 69%)"],
        ["#4100a8", "hwb(from #fff 263 0% 34%)"],
        ["#f3c9ff", "hwb(from #fff 287 79% 0%)"],
        ["#ff24d7", "hwb(from #fff 311 14% 0%)"],
        ["#820036", "hwb(from #fff 335 0% 49%)"],
        // keywords
        ["#bad455", "hwb(from #bad455 h w b)"],
        ["#bad455", "hwb(from #bad455 h w b / alpha)"],
        // calc(...)
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
        "hwb(55% 0 0)",
        "hwb(from #bad455 0% w b)",
    ];
    for s in test_data {
        assert!(parse(s).is_err(), "{s:?}");
    }
}
