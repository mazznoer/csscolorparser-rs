use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        "hsl(0 48% 83%)",
        "hsl(17 73% 13%)",
        "hsl(35 40% 84%)",
        "hsl(53 88% 21%)",
        "hsl(71 11% 45%)",
        "hsl(89 12% 89%)",
        "hsl(107 49% 68%)",
        "hsl(125 96% 72%)",
        "hsl(143 15% 92%)",
        "hsl(161 80% 93%)",
        "hsl(179 45% 76%)",
        "hsl(197 99% 84%)",
        "hsl(215 33% 15%)",
        "hsl(233 69% 59%)",
        "hsl(251 34% 46%)",
        "hsl(269 43% 18%)",
        "hsl(287 89% 69%)",
        "hsl(305 87% 36%)",
        "hsl(323 97% 26%)",
        "hsl(341 61% 66%)",
        "hsl(359 15% 74%)",
    ];

    for s in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(s, c.unwrap().to_css_hsl().to_string(), "{s:?}");
    }
}

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#000000", "hsl(none none none)"],
        ["#00000000", "hsl(197 0 0 / none)"],

        // bare number
        ["#421f1f", "hsl(0 37 19)"],
        ["#88481b", "hsl(25 67 32)"],
        ["#b49f27", "hsl(51 64 43)"],
        ["#2f3a13", "hsl(76 51 15)"],
        ["#2a481e", "hsl(102 42 20)"],
        ["#3cf655", "hsl(128 91 60)"],
        ["#8ce3bc", "hsl(153 61 72)"],
        ["#388584", "hsl(179 41 37)"],
        ["#2f5a79", "hsl(205 44 33)"],
        ["#17256e", "hsl(230 66 26)"],
        ["#815bec", "hsl(256 79 64)"],
        ["#dfa5f8", "hsl(282 86 81)"],
        ["#6a2963", "hsl(307 44 29)"],
        ["#d8c0cb", "hsl(333 24 80)"],
        ["#fdfcfc", "hsl(359 11 99)"],

        // angle suffix
        ["#1c415a", "hsl(0.565turn 53 23)"],
        ["#dceed8", "hsl(120grad 40 89)"],
        ["#af088d", "hsl(347grad 91 36)"],
        ["#ebfed2", "hsl(0.239turn 95 91)"],
        ["#ab6d93", "hsl(323deg 27 55)"],
        ["#433743", "hsl(0.842turn 10 24)"],
        ["#cf2639", "hsl(0.981turn 69 48)"],
        ["#c7a957", "hsl(44deg 50 56)"],
        ["#3aa305", "hsl(100deg 94 33)"],
        ["#45565f", "hsl(0.557turn 16 32)"],
        ["#76919e", "hsl(221grad 17 54)"],
        ["#5f3321", "hsl(20grad 49 25)"],
        ["#c9cfab", "hsl(1.197rad 27 74)"],
        ["#91baf8", "hsl(216deg 88 77)"],
        ["#fce9e9", "hsl(0.003turn 76 95)"],
        ["#43143c", "hsl(343grad 54 17)"],
        ["#a4d0ab", "hsl(143grad 32 73)"],
        ["#0e565d", "hsl(185deg 73 21)"],
        ["#e46caf", "hsl(363grad 69 66)"],
        ["#a0caa7", "hsl(2.281rad 29 71)"],
        ["#19578f", "hsl(3.640rad 70 33)"],
        ["#ebd57f", "hsl(0.838rad 73 71)"],
        ["#cc75b9", "hsl(0.870turn 46 63)"],
        ["#c8a4d6", "hsl(4.937rad 38 74)"],
        ["#532228", "hsl(0.980turn 42 23)"],
        ["#40b55f", "hsl(2.371rad 48 48)"],
        ["#def575", "hsl(1.236rad 86 71)"],
        ["#15291d", "hsl(2.539rad 33 12)"],
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
        ["#000000", "hsl(from #bad455 none none none)"],
        ["#bad45500", "hsl(from #bad455 h s l / none)"],

        // bare number
        ["#ea4343", "hsl(from #fff 0 80 59)"],
        ["#27170c", "hsl(from #fff 25 52 10)"],
        ["#f2e497", "hsl(from #fff 51 78 77)"],
        ["#58633b", "hsl(from #fff 76 25 31)"],
        ["#3f9a18", "hsl(from #fff 102 73 35)"],
        ["#46724c", "hsl(from #fff 128 24 36)"],
        ["#e6efeb", "hsl(from #fff 153 21 92)"],
        ["#19e1de", "hsl(from #fff 179 80 49)"],
        ["#1b75b6", "hsl(from #fff 205 74 41)"],
        ["#323f81", "hsl(from #fff 230 44 35)"],
        ["#241e33", "hsl(from #fff 256 26 16)"],
        ["#b083c3", "hsl(from #fff 282 35 64)"],
        ["#9c3590", "hsl(from #fff 307 49 41)"],
        ["#e1adc4", "hsl(from #fff 333 47 78)"],
        ["#f8595b", "hsl(from #fff 359 92 66)"],

        // percentage
        ["#b57d7d", "hsl(from #fff 0 27% 60%)"],
        ["#bc6229", "hsl(from #fff 23 64% 45%)"],
        ["#cca724", "hsl(from #fff 47 70% 47%)"],
        ["#ddfd4e", "hsl(from #fff 71 98% 65%)"],
        ["#a1d37e", "hsl(from #fff 95 49% 66%)"],
        ["#5ad058", "hsl(from #fff 119 56% 58%)"],
        ["#40c975", "hsl(from #fff 143 56% 52%)"],
        ["#579e8f", "hsl(from #fff 167 29% 48%)"],
        ["#92e0f2", "hsl(from #fff 191 78% 76%)"],
        ["#b9c2d0", "hsl(from #fff 215 20% 77%)"],
        ["#191b6b", "hsl(from #fff 239 62% 26%)"],
        ["#510ac2", "hsl(from #fff 263 90% 40%)"],
        ["#5a3c62", "hsl(from #fff 287 24% 31%)"],
        ["#c1a4bc", "hsl(from #fff 311 19% 70%)"],
        ["#aa0349", "hsl(from #fff 335 96% 34%)"],

        // keywords
        ["#bad455", "hsl(from #bad455 h s l)"],
        ["#bad455", "hsl(from #bad455 h s l / alpha)"],

        // calc(...)
        ["#266573", "hsl(from #fff calc(7 * 13 + 100) calc(5 * 20 / 2) calc(6 * 5))"],
        ["#55bfd4", "hsl(from #bad455 calc(h + s + l) s l)"],
        ["#b9db35", "hsl(from #bad455 h calc(s + 10) calc(l - 5))"],
        ["#80ff00", "hsl(from #f00 calc(9e1) s l)"],
        ["#bad455", "hsl(from #fff calc(18.79797 / 3 + 198.05238 / 3) calc(20.16437759098393 + 4.129543602599515 + 70.66097761283312 / 2) calc(2 * (28.554087 + 0.56356)))"],
        ["#bad455", "hsl(from #fff calc(5 * 144.5669 / (20 / 2)) calc((0.330781645625726 + 4.669218354374274) * 1.1924882 * 10) calc(14.827014 + 97.819461 - 54.411181))"],
        ["#bad455", "hsl(from #fff calc(255.69098 * 0.5 - 27.78102 / 0.5) calc(22.285836693884615 * 5 - 4 * 12.951193367355769) calc(873.52941 / 3 / 5))"],
        ["#bad455", "hsl(from #fff calc(5 * 72.28345 / (0.25 * 20)) calc(178.87323 / (12 / 4)) calc(33.812941 + 1.394337 + 23.028016))"],
        ["#bad455", "hsl(from #fff calc(0.5 * 722.8345 / (5 * 1)) calc((6.453243862566449 + 23.358961137433553) * (4.166020240728482 - 2.166020240728482)) calc(582.35294 / (0.25 * 40)))"],
        ["#bad455", "hsl(from #e17337 calc(3061.4173228346463 / 2 / h) calc(s * 1.248661523462753 - 98.00388098338202 / 3) calc(l * 0.106071428571429 * 10))"],
        ["#bad455", "hsl(from #adedc9 calc(58.27771099178472 + 72.02070516957522 - (h - 88.2350484055692)) calc(7.483701505844654 + 9.76553997796077 + s - 21.624828338265516) calc(l - (2.328511238230386 + 19.828351506867644)))"],
        ["#bad455", "hsl(from #1d57d0 calc((14.50521288333021 + 21.63651940013436) / (h - 220.05865921787708)) calc(128.08018826306704 - 19.569192197721435 - (-26.640843240531865 + s)) calc(l + 47.0588235294118 / 4))"],
        ["#bad455", "hsl(from #ba09ae calc(h / (5 * 0.84132048299546)) calc(0.5 * 300.7872878295414 - s) calc(l * (0.662910824622575 + 0.860166098454348)))"],
        ["#bad455", "hsl(from #b792b2 calc(h - (369.044137212788 - 133.219493671609)) calc(s / 0.177227861631552 - 278.59281313186955 / 5) calc(-6.274509803921553 + l))"],
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
        "hsl(0% 35 60)",
        "hsl(from #bad455 0% s l)",
    ];

    for s in test_data {
        assert!(parse(s).is_err(), "{s:?}");
    }
}
