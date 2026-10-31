use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        ["#765400", "lab(38.44 8.73 60.33)"],
        ["#00dcb1", "lab(75.22 -82.94 2.86)"],
        ["#be0020", "lab(22.82 120.44 25.72)"],
        ["#9aacc8", "lab(69.75 -2.02 -16.62)"],
        ["#002d20", "lab(6.23 -79.5 -10.83)"],
        ["#f90000", "lab(43.14 107.06 119.25)"],
        ["#0098ff", "lab(48.3 -87.94 -114.5)"],
        ["#50593c", "lab(36.3 -7.83 15.58)"],
        ["#00a3ff", "lab(61.37 -19.6 -72.08)"],
        ["#4b5f00", "lab(37.11 -19.68 77.5)"],
        ["#ff009e", "lab(73.32 105.19 16.2)"],
        ["#005ebd", "lab(28.62 -81.68 -76.06)"],
        ["#ffc100", "lab(90.88 43.51 119.32)"],
        ["#5a0000", "lab(4.5 61.43 123.56)"],
        ["#a40000", "lab(25.8 83.7 61.52)"],
        ["#ff0044", "lab(65.09 96.52 55.35)"],
        ["#c40016", "lab(25.56 116.81 38.38)"],
        ["#00e158", "lab(75.93 -97.22 47.69)"],
        ["#ddeaff", "lab(96.28 22.2 -71.83)"],
        ["#ffd663", "lab(94.47 32.05 70.52)"],
        ["#5d00ff", "lab(36.38 87.93 -122.38)"],
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        let c = c.unwrap();
        assert_eq!(s, c.to_css_lab().to_string(), "{s:?}");
        assert_eq!(hex, c.to_string(), "{s:?}");
    }
}

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "lab(none none none / none)"],

        // percentage
        ["#ff86ad", "lab(98% 83% 35%)"],
        ["#00ffff", "lab(84% -92% -31%)"],
        ["#00d883", "lab(73% -73% 19%)"],
        ["#00d5ff", "lab(67% -99% -96%)"],
        ["#ffcc00", "lab(88% 16% 96%)"],
        ["#ff8e0e", "lab(71% 32% 60%)"],
        ["#009600", "lab(50% -78% 45%)"],
        ["#2a3400", "lab(19% -16% 66%)"],
        ["#ff79ff", "lab(78% 67% -72%)"],
        ["#8ea200", "lab(63% -17% 73%)"],
        ["#695687", "lab(40% 13% -20%)"],
        ["#0050d7", "lab(27% -17% -75%)"],
        ["#ff8c8f", "lab(82% 56% 28%)"],
        ["#b59fff", "lab(72% 25% -51%)"],
        //["#88ca55", "lab(75% -29% 40%)"],
        ["#51582b", "lab(36% -7% 20%)"],
        ["#6a0000", "lab(16% 40% 72%)"],
        ["#00e7ff", "lab(84% -21% -79%)"],
        ["#b3cc00", "lab(78% -20% 73%)"],
        ["#6e0000", "lab(4% 73% 89%)"],
        ["#ad4639", "lab(44% 34% 24%)"],
        ["#004e00", "lab(25% -61% 91%)"],
        ["#003ea2", "lab(21% -12% -58%)"],
        ["#a18b4d", "lab(59% 2% 29%)"],
        ["#00ebe6", "lab(79% -72% -16%)"],
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
        ["#000000", "lab(from #bad455 none none none)"],
        ["#bad45500", "lab(from #bad455 l a b / none)"],

        // bare number
        ["#ff0000", "lab(from #fff 57.2 90.45 98.64)"],
        ["#3b0000", "lab(from #fff 1.71 39.72 36.87)"],
        ["#32000c", "lab(from #fff 5.63 28.11 4.15)"],
        ["#00c8a5", "lab(from #fff 70.95 -59.12 3.21)"],
        ["#4b3c0d", "lab(from #fff 26.19 2.98 29.93)"],
        ["#00f200", "lab(from #fff 82.46 -90.87 123.5)"],
        ["#003dd0", "lab(from #fff 12.15 -67.07 -114.86)"],
        ["#ffbe7f", "lab(from #fff 90.54 46.22 53.64)"],
        ["#ff57ff", "lab(from #fff 72.98 94.35 -96.25)"],
        ["#006e00", "lab(from #fff 35.78 -96.97 67.05)"],
        ["#0076ff", "lab(from #fff 52.36 29.86 -120.97)"],
        ["#460067", "lab(from #fff 9.9 61.58 -52.79)"],
        ["#d66d00", "lab(from #fff 57.78 37.99 85.95)"],
        ["#00cfff", "lab(from #fff 66.05 -121.45 -90.23)"],
        ["#009dff", "lab(from #fff 60.32 -9.75 -93.04)"],

        // percentage
        ["#8ac17a", "lab(from #fff 73% -23% 24%)"],
        ["#bd0034", "lab(from #fff 29% 80% 13%)"],
        ["#938163", "lab(from #fff 55% 3% 15%)"],
        ["#240020", "lab(from #fff 3% 19% -11%)"],
        ["#009fff", "lab(from #fff 65% 6% -66%)"],
        ["#009eb8", "lab(from #fff 51% -93% -30%)"],
        ["#caac6b", "lab(from #fff 72% 4% 30%)"],
        ["#0062cd", "lab(from #fff 32% -40% -64%)"],
        ["#786800", "lab(from #fff 44% -3% 92%)"],
        ["#840027", "lab(from #fff 17% 66% 5%)"],
        ["#ff9e00", "lab(from #fff 83% 45% 80%)"],
        ["#ffabb9", "lab(from #fff 86% 43% 14%)"],
        ["#007900", "lab(from #fff 40% -75% 68%)"],
        ["#003cb5", "lab(from #fff 8% -86% -85%)"],
        ["#590000", "lab(from #fff 6% 51% 50%)"],

        // keywords
        ["#bad455", "lab(from #bad455 l a b)"],
        ["#bad455", "lab(from #bad455 l a b / alpha)"],

        // calc(...)
        ["#bad455", "lab(from #bad455 calc(l) calc(a) calc(b) / calc(alpha))"],
        ["#bad455", "lab(from #fff calc(12.7768 - 4.0709 + 134.7535 - 62.3482) calc(0.433 - 9.256 - 10.536 - 2.762) calc(28.9392 / 0.25 / 2))"],
        ["#bad455", "lab(from #fff calc(55.1726 + 6.2836 + 19.655) calc(-20.184 - 1.937) calc(529.3 / 5 - 0.25 * 191.9264))"],
        ["#bad455", "lab(from #fff calc(0.25 * 81.1112 * 4) calc(-442.42 * 0.25 / 5) calc((285.6597 - 54.1461) / (5.9708 - 1.9708)))"],
        ["#bad455", "lab(from #fff calc(1 * 2 * 20.2778 / 0.5) calc(-208.23 / 10 - 0.674 - 0.624) calc(4 * 14.4696))"],
        ["#bad455", "lab(from #fff calc(19.6732 - 7.1541 + 87.6369 - 19.0448) calc(-176.968 / 4 * (0.27 + 0.23)) calc(75.0165 - 17.1381))"],
        ["#bad455", "lab(from #d1fae8 calc(-8.0965 - 5.7878 + l) calc((-50.1415 + a) / 3) calc(b + 80.57533 - 26.74748))"],
        ["#bad455", "lab(from #3e57ba calc(27.7221 + 14.0113 + l) calc(-9.3403 - (a - 3.5619)) calc(114.1697 + b))"],
        ["#bad455", "lab(from #b0f42d calc(20.2778 / 0.5 * (l - 87.2388)) calc(a + 56.916 / 3) calc(151.4243 - b - 15.0695))"],
        ["#bad455", "lab(from #534387 calc(47.9876 + 0.3864 + l) calc(a - (5.4557 + 36.7124)) calc(148.2461 + b - (26.6734 + 27.401)))"],
        ["#bad455", "lab(from #2687fb calc(148.906 / 10 + l + 10.4885) calc(1.72178 - 9.0715 - 9.53519 - a) calc((-62.7927 - b) * (5.3646 + 9.105)))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
