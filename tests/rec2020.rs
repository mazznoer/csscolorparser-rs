use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(rec2020 none none none / none)"],

        // bare number
        ["#800531", "color(rec2020 0.352 0.075 0.133)"],
        ["#00dd4d", "color(rec2020 0.096 0.801 0.332)"],
        ["#c54daf", "color(rec2020 0.622 0.309 0.628)"],
        ["#9019ad", "color(rec2020 0.428 0.137 0.61)"],
        ["#d290ad", "color(rec2020 0.716 0.545 0.638)"],
        ["#cc7fa8", "color(rec2020 0.679 0.48 0.612)"],
        ["#8222cb", "color(rec2020 0.397 0.15 0.734)"],
        ["#a94539", "color(rec2020 0.511 0.26 0.183)"],
        ["#009491", "color(rec2020 0.174 0.507 0.519)"],
        ["#76a700", "color(rec2020 0.478 0.601 0.017)"],
        ["#babc9f", "color(rec2020 0.697 0.707 0.596)"],
        ["#0063c9", "color(rec2020 0.087 0.318 0.729)"],
        ["#00cf93", "color(rec2020 0.185 0.745 0.55)"],
        ["#00a035", "color(rec2020 0.239 0.558 0.215)"],
        ["#9500d3", "color(rec2020 0.452 0.098 0.766)"],
        ["#00777e", "color(rec2020 0.016 0.389 0.437)"],
        ["#ff00ed", "color(rec2020 0.822 0.066 0.878)"],
        ["#c5fc9e", "color(rec2020 0.829 0.968 0.631)"],
        ["#00cec0", "color(rec2020 0.033 0.74 0.722)"],
        ["#00b4c0", "color(rec2020 0.091 0.634 0.712)"],
        ["#008838", "color(rec2020 0.097 0.458 0.201)"],
        ["#e06e32", "color(rec2020 0.719 0.428 0.198)"],
        ["#00ffdc", "color(rec2020 0.091 0.964 0.85)"],
        ["#e3ad24", "color(rec2020 0.788 0.659 0.222)"],
        ["#733fe2", "color(rec2020 0.378 0.225 0.831)"],

        // percentage
        ["#ff8087", "color(rec2020 99% 54% 50%)"],
        ["#e55cbd", "color(rec2020 74% 38% 69%)"],
        ["#0070f0", "color(rec2020 8% 37% 89%)"],
        ["#cdcdc5", "color(rec2020 78% 78% 75%)"],
        ["#ff57bc", "color(rec2020 92% 41% 69%)"],
        //["#8cff00", "color(rec2020 71% 99% 3%)"],
        ["#125474", "color(rec2020 16% 26% 39%)"],
        ["#00f9be", "color(rec2020 20% 92% 73%)"],
        ["#dacd00", "color(rec2020 80% 78% 21%)"],
        ["#009b74", "color(rec2020 3% 53% 41%)"],
        ["#aa9185", "color(rec2020 59% 53% 48%)"],
        ["#00c669", "color(rec2020 25% 71% 40%)"],
        ["#4647f5", "color(rec2020 29% 24% 91%)"],
        ["#527156", "color(rec2020 31% 38% 29%)"],
        ["#7d00d4", "color(rec2020 37% 1% 77%)"],
        ["#ff76c0", "color(rec2020 93% 50% 71%)"],
        ["#fad642", "color(rec2020 91% 83% 33%)"],
        ["#ff007c", "color(rec2020 78% 0% 42%)"],
        ["#ff809e", "color(rec2020 88% 52% 58%)"],
        ["#00df96", "color(rec2020 1% 81% 57%)"],
        ["#ff9fe6", "color(rec2020 90% 63% 87%)"],
        ["#94b6a7", "color(rec2020 59% 67% 62%)"],
        ["#00a200", "color(rec2020 11% 56% 8%)"],
        ["#74dc00", "color(rec2020 58% 82% 6%)"],
        ["#0962ab", "color(rec2020 21% 32% 61%)"],
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
        ["#00000000", "color(from #bad455 rec2020 none none none / none)"],

        // bare number
        ["#00aef1", "color(from #fff rec2020 0.038 0.611 0.908)"],
        ["#c9e63a", "color(from #fff rec2020 0.793 0.877 0.32)"],
        ["#d2d5b6", "color(from #fff rec2020 0.803 0.814 0.698)"],
        ["#dcaf79", "color(from #fff rec2020 0.773 0.664 0.457)"],
        ["#ff5fad", "color(from #fff rec2020 0.905 0.426 0.632)"],
        ["#fa0072", "color(from #fff rec2020 0.77 0.133 0.385)"],
        ["#9e00ff", "color(from #fff rec2020 0.489 0.048 0.949)"],
        ["#00c6cb", "color(from #fff rec2020 0.102 0.711 0.759)"],
        ["#5f4e1c", "color(from #fff rec2020 0.286 0.248 0.086)"],
        ["#ff9949", "color(from #fff rec2020 0.897 0.604 0.302)"],
        ["#ff33ff", "color(from #fff rec2020 0.865 0.321 0.979)"],
        ["#ff3c00", "color(from #fff rec2020 0.912 0.342 0.041)"],
        ["#cb8c4e", "color(from #fff rec2020 0.679 0.523 0.291)"],
        ["#e442bb", "color(from #fff rec2020 0.725 0.307 0.676)"],
        ["#dcfefd", "color(from #fff rec2020 0.906 0.987 0.989)"],

        // percentage
        ["#bfef00", "color(from #fff rec2020 78% 91% 20%)"],
        ["#4b4c65", "color(from #fff rec2020 24% 24% 33%)"],
        ["#c4a5c1", "color(from #fff rec2020 70% 62% 72%)"],
        ["#e66ae7", "color(from #fff rec2020 76% 43% 86%)"],
        ["#e3ad62", "color(from #fff rec2020 79% 66% 38%)"],
        ["#99e4c9", "color(from #fff rec2020 69% 86% 77%)"],
        ["#0080c3", "color(from #fff rec2020 27% 44% 71%)"],
        ["#ce005c", "color(from #fff rec2020 62% 14% 30%)"],
        ["#d5de9d", "color(from #fff rec2020 82% 85% 61%)"],
        ["#a005b1", "color(from #fff rec2020 48% 13% 63%)"],
        ["#00abca", "color(from #fff rec2020 4% 60% 75%)"],
        ["#0058bd", "color(from #fff rec2020 16% 28% 68%)"],
        ["#92ffaf", "color(from #fff rec2020 73% 98% 69%)"],
        ["#da0071", "color(from #fff rec2020 66% 14% 38%)"],
        ["#006666", "color(from #fff rec2020 17% 33% 34%)"],

        // keywords
        ["#bad455", "color(from #bad455 rec2020 r g b)"],
        ["#f851c6", "color(from #bad455 rec2020 g b r / alpha)"],

        // calc(...)
        ["#bad455", "color(from #bad455 rec2020 calc(r) calc(g) calc(b))"],
        ["#f851c6", "color(from #bad455 rec2020 calc(g) calc(b) calc(r))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
