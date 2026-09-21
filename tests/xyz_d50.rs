use csscolorparser::parse;

#[test]
fn one() {
    let test_data = [
        // Absolute
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
        // Absolute (percentage)
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
        // Relative
        ["#bad455", "color(from #bad455 xyz-d50 x y z)"],
        ["#bad455", "color(from #bad455 xyz-d50 x y z / alpha)"],
    ];
    for [hex, xyz_d50] in test_data {
        let c = parse(xyz_d50).unwrap();
        assert_eq!(hex, c.to_css_hex().to_string(), "{:?}", xyz_d50);
    }
}
