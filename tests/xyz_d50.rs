use csscolorparser::parse;

#[test]
fn absolute() {
    let test_data = [
        // none
        ["#00000000", "color(xyz-d50 none none none / none)"],
        // bare number
        ["#ff53d1", "color(xyz-d50 1.103 0.6 0.495)"],
        ["#ff0069", "color(xyz-d50 0.837 0.367 0.118)"],
        ["#f1bcb1", "color(xyz-d50 0.641 0.584 0.375)"],
        ["#2a1808", "color(xyz-d50 0.014 0.012 0.003)"],
        ["#b24d00", "color(xyz-d50 0.217 0.15 -0.02)"],
        ["#4c3c00", "color(xyz-d50 0.047 0.048 -0.006)"],
        ["#29310c", "color(xyz-d50 0.022 0.027 0.006)"],
        ["#aee97d", "color(xyz-d50 0.529 0.692 0.232)"],
        ["#00cc47", "color(xyz-d50 0.225 0.429 0.103)"],
        ["#006e17", "color(xyz-d50 0.017 0.09 0.02)"],
        ["#008d64", "color(xyz-d50 0.013 0.144 0.114)"],
        ["#92b9b9", "color(xyz-d50 0.383 0.443 0.397)"],
        ["#216571", "color(xyz-d50 0.08 0.106 0.131)"],
        ["#c8ffff", "color(xyz-d50 0.818 0.925 1.001)"],
        ["#656f7a", "color(xyz-d50 0.147 0.156 0.157)"],
        ["#005dff", "color(xyz-d50 0.079 0.083 0.864)"],
        ["#0083ff", "color(xyz-d50 0.381 0.282 1.761)"],
        ["#593b84", "color(xyz-d50 0.094 0.068 0.171)"],
        ["#a400b8", "color(xyz-d50 0.221 0.094 0.346)"],
        ["#ff7dd4", "color(xyz-d50 0.623 0.416 0.503)"],
        ["#ff5fc8", "color(xyz-d50 0.929 0.526 0.447)"],
        // percentage
        ["#7e0000", "color(xyz-d50 7% 1% -1%)"],
        ["#9d1413", "color(xyz-d50 15% 8% 1%)"],
        ["#ff6200", "color(xyz-d50 56% 35% 2%)"],
        ["#ffc000", "color(xyz-d50 97% 77% 2%)"],
        //["#a78500", "color(xyz-d50 25% 25% -2%)"],
        ["#aec430", "color(xyz-d50 40% 49% 8%)"],
        ["#c6d1ba", "color(xyz-d50 56% 61% 42%)"],
        ["#61a669", "color(xyz-d50 22% 31% 14%)"],
        ["#003e16", "color(xyz-d50 1% 3% 1%)"],
        ["#006e49", "color(xyz-d50 0% 8% 6%)"],
        ["#007b8e", "color(xyz-d50 2% 11% 21%)"],
        ["#00c1e7", "color(xyz-d50 24% 39% 62%)"],
        ["#004bc7", "color(xyz-d50 0% 3% 41%)"],
        ["#00302b", "color(xyz-d50 1% 2% 2%)"],
        ["#7fcbff", "color(xyz-d50 55% 57% 120%)"],
        ["#0035e1", "color(xyz-d50 6% 4% 54%)"],
        ["#9243ff", "color(xyz-d50 35% 19% 102%)"],
        ["#39373f", "color(xyz-d50 4% 4% 4%)"],
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
        ["#000000", "color(from #bad455 xyz-d50 none none none)"],
        ["#bad45500", "color(from #bad455 xyz-d50 x y z / none)"],
        // bare number
        ["#281c20", "color(from #fff xyz-d50 0.016 0.014 0.012)"],
        ["#8c6665", "color(from #fff xyz-d50 0.184 0.162 0.109)"],
        ["#af0000", "color(from #fff xyz-d50 0.168 0.067 -0.022)"],
        ["#583c00", "color(from #fff xyz-d50 0.06 0.054 0.005)"],
        ["#d6dc4d", "color(from #fff xyz-d50 0.581 0.668 0.132)"],
        ["#003200", "color(from #fff xyz-d50 0.004 0.019 -0.02)"],
        ["#2d8455", "color(from #fff xyz-d50 0.114 0.178 0.088)"],
        ["#00d2b3", "color(from #fff xyz-d50 0.273 0.47 0.384)"],
        ["#adf2f7", "color(from #fff xyz-d50 0.657 0.785 0.758)"],
        //["#006f8d", "color(from #fff xyz-d50 0.096 0.128 0.204)"],
        ["#00f8ff", "color(from #fff xyz-d50 0.57 0.728 2.924)"],
        ["#004bf0", "color(from #fff xyz-d50 0.095 0.074 0.625)"],
        ["#554763", "color(from #fff xyz-d50 0.082 0.073 0.097)"],
        ["#5e0051", "color(from #fff xyz-d50 0.049 0.008 0.058)"],
        ["#90002e", "color(from #fff xyz-d50 0.102 0.019 0.017)"],
        // percentage
        ["#6c2540", "color(from #fff xyz-d50 8% 5% 4%)"],
        ["#ff001f", "color(from #fff xyz-d50 44% 14% 1%)"],
        ["#ffae5a", "color(from #fff xyz-d50 96% 71% 14%)"],
        ["#ae5d00", "color(from #fff xyz-d50 22% 17% -2%)"],
        ["#e3f700", "color(from #fff xyz-d50 68% 83% 4%)"],
        ["#005200", "color(from #fff xyz-d50 1% 5% -3%)"],
        ["#00aa00", "color(from #fff xyz-d50 6% 24% 3%)"],
        ["#00e7b0", "color(from #fff xyz-d50 14% 48% 38%)"],
        ["#00feff", "color(from #fff xyz-d50 26% 63% 109%)"],
        ["#0080ff", "color(from #fff xyz-d50 4% 12% 76%)"],
        ["#0067fa", "color(from #fff xyz-d50 6% 9% 69%)"],
        ["#353848", "color(from #fff xyz-d50 4% 4% 5%)"],
        ["#ffc9ff", "color(from #fff xyz-d50 139% 97% 248%)"],
        ["#ff29ff", "color(from #fff xyz-d50 168% 84% 169%)"],
        ["#d0004d", "color(from #fff xyz-d50 24% 6% 5%)"],
        // keywords
        ["#bad455", "color(from #bad455 xyz-d50 x y z)"],
        ["#bad455", "color(from #bad455 xyz-d50 x y z / alpha)"],
        // calc(...)
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
