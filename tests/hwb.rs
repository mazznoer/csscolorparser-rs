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
    #[rustfmt::skip]
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

        // angle suffix
        ["#eee3ff", "hwb(263deg 89 0)"],
        ["#ff8cf2", "hwb(341grad 55 0)"],
        ["#8a4a00", "hwb(0.090turn 0 46)"],
        ["#787bff", "hwb(265grad 47 0)"],
        ["#ddffab", "hwb(1.472rad 67 0)"],
        ["#001421", "hwb(227grad 0 87)"],
        ["#ca00de", "hwb(5.144rad 0 13)"],
        ["#92ff8c", "hwb(0.325turn 55 0)"],
        ["#fffd33", "hwb(0.165turn 20 0)"],
        ["#ff858e", "hwb(6.200rad 52 0)"],
        ["#b0ecff", "hwb(3.399rad 69 0)"],
        ["#13001c", "hwb(0.782turn 0 89)"],
        ["#001f1e", "hwb(3.109rad 0 88)"],
        ["#f8ffd6", "hwb(0.194turn 84 0)"],
        ["#00a368", "hwb(0.440turn 0 36)"],
        ["#302800", "hwb(0.136turn 0 81)"],
        ["#5c009c", "hwb(0.765turn 0 39)"],
        ["#a3ffb7", "hwb(148grad 64 0)"],
        ["#4f85ff", "hwb(0.616turn 31 0)"],
        ["#0c3600", "hwb(1.851rad 0 79)"],
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

        // angle suffix
        ["#eee3ff", "hwb(from #fff 263deg 89 0)"],
        ["#ff8cf2", "hwb(from #fff 341grad 55 0)"],
        ["#8a4a00", "hwb(from #fff 0.090turn 0 46)"],
        ["#ddffab", "hwb(from #fff 1.472rad 67 0)"],

        // keywords
        ["#bad455", "hwb(from #bad455 h w b)"],
        ["#bad455", "hwb(from #bad455 h w b / alpha)"],

        // calc(...)
        ["#bad455", "hwb(from #bad455 calc(h) calc(w) calc(b))"],
        ["#0d94b3", "hwb(from #bad455 calc(7 * 13 + 100) calc(100 - 95) calc(90 / 3))"],
        ["#ae9d46", "hwb(from #bad455 calc(h * 0.7) calc(w - 6) calc(b + 15))"],
        ["#bad455", "hwb(from #fff calc(2891.338848285279 / 4 / (11.50075771713361 - 1.50075771713361)) calc(8.5523 - 2.7371 + 14.5751 + 12.943) calc(5.231370661117048 * 3 + 1.168588016648853))"],
        ["#bad455", "hwb(from #fff calc(289.13388482852787 / 4) calc(266.6664 / 2 / (2 / 0.5)) calc(22.483599999999996 / 4 * 3))"],
        ["#bad455", "hwb(from #fff calc(163.13737307998738 / 2 - 18.57043066572344 / 2) calc(11.1111 * 3 * 1) calc(0.32003195398167 + 3.742211523674281 + 3.993757668943228 + 8.806698853400817))"],
        ["#bad455", "hwb(from #fff calc(6 / 2 * (45.43464438465629 - 21.34015398227897)) calc(188.0636 / 4 - 41.0478 / 3) calc(50.58809999999999 / (10.494590842687172 - 7.494590842687172)))"],
        ["#bad455", "hwb(from #fff calc(105.63938508446623 - 133.42365550933707 * 0.25) calc(0.25 * 2 * 3 * 22.2222) calc(10.577866724271342 + 15.545074437989538 - (10.878766243420646 - 1.618525081159765)))"],
        ["#bad455", "hwb(from #bf376e calc(24192 / h) calc(79.23 - 25.25 - (w - 1.02)) calc(0.25 * 34.88 + 207 / b))"],
        ["#bad455", "hwb(from #cfde35 calc(1 + 6 + h) calc(173.25 / 0.25 / w) calc(b + 12.29 - 8.29))"],
        ["#bad455", "hwb(from #211038 calc(0.25 * 318.92 - 2056.18 / h) calc(17.09 + 9.91 + w) calc((7.85 + 0.65) / (39 / b)))"],
        ["#bad455", "hwb(from #726075 calc(125.96 * 0.25 - 250.49 + h) calc(1254 / w) calc(14.22 + 56.78 - b))"],
        ["#bad455", "hwb(from #84b53f calc(2455.65 / h + 30.08 + 13.03) calc(77.3 - w - 77.2 / 4) calc(11.5 / 0.25 - b))"],
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
        // invalid w & b
        "hwb(from #bad455 h 0.01rad 10deg)",
    ];

    for s in test_data {
        assert!(parse(s).is_err(), "{s:?}");
    }
}
