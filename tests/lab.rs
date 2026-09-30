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
fn relative() {
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
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
