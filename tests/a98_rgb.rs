use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(a98-rgb none none none / none)"],

        // bare number
        ["#b7dfbc", "color(a98-rgb 0.76 0.869 0.738)"],
        ["#73c400", "color(a98-rgb 0.561 0.765 0.031)"],
        ["#d953aa", "color(a98-rgb 0.742 0.328 0.651)"],
        ["#ff935b", "color(a98-rgb 0.911 0.573 0.369)"],
        ["#756d00", "color(a98-rgb 0.448 0.424 0.068)"],
        ["#e47fb5", "color(a98-rgb 0.802 0.496 0.698)"],
        ["#3864b8", "color(a98-rgb 0.287 0.392 0.707)"],
        //["#b326b6", "color(a98-rgb 0.603 0.165 0.697)"],
        ["#f465a0", "color(a98-rgb 0.841 0.397 0.615)"],
        ["#792300", "color(a98-rgb 0.412 0.155 0.031)"],
        ["#acb900", "color(a98-rgb 0.685 0.721 0.071)"],
        ["#702755", "color(a98-rgb 0.383 0.17 0.332)"],
        ["#0088ac", "color(a98-rgb 0.061 0.527 0.663)"],
        ["#92668f", "color(a98-rgb 0.526 0.401 0.549)"],
        ["#004a57", "color(a98-rgb 0.144 0.297 0.341)"],
        ["#8308f0", "color(a98-rgb 0.439 0.064 0.922)"],
        ["#00af72", "color(a98-rgb 0.301 0.681 0.459)"],
        ["#83c279", "color(a98-rgb 0.594 0.757 0.487)"],
        ["#a48e34", "color(a98-rgb 0.613 0.551 0.241)"],
        ["#2e9200", "color(a98-rgb 0.353 0.567 0.117)"],
        ["#c8dd9f", "color(a98-rgb 0.805 0.864 0.632)"],
        //["#18f88f", "color(a98-rgb 0.555 0.971 0.582)"],
        //["#ff12ad", "color(a98-rgb 0.956 0.098 0.662)"],
        ["#ff398c", "color(a98-rgb 0.868 0.234 0.536)"],
        ["#006f54", "color(a98-rgb 0.012 0.435 0.336)"],

        // percentage
        ["#552a9d", "color(a98-rgb 30% 18% 60%)"],
        ["#ff0aba", "color(a98-rgb 91% 7% 71%)"],
        ["#f6d792", "color(a98-rgb 93% 84% 58%)"],
        ["#6cc1a1", "color(a98-rgb 54% 75% 63%)"],
        ["#081b33", "color(a98-rgb 9% 13% 21%)"],
        ["#0530f2", "color(a98-rgb 12% 20% 93%)"],
        ["#00ac89", "color(a98-rgb 24% 67% 54%)"],
        ["#f2c600", "color(a98-rgb 90% 77% 15%)"],
        ["#0090c2", "color(a98-rgb 16% 56% 75%)"],
        ["#ab9340", "color(a98-rgb 64% 57% 28%)"],
        ["#00bc32", "color(a98-rgb 6% 73% 26%)"],
        ["#31bcbe", "color(a98-rgb 44% 73% 74%)"],
        ["#d5d5f9", "color(a98-rgb 83% 83% 97%)"],
        ["#b54672", "color(a98-rgb 62% 28% 44%)"],
        ["#bf5900", "color(a98-rgb 66% 35% 5%)"],
        ["#ff8e85", "color(a98-rgb 93% 55% 52%)"],
        ["#004b00", "color(a98-rgb 16% 30% 7%)"],
        ["#c5b4af", "color(a98-rgb 75% 70% 68%)"],
        ["#55bc6c", "color(a98-rgb 49% 73% 44%)"],
        ["#88b9ce", "color(a98-rgb 59% 72% 80%)"],
        ["#006e00", "color(a98-rgb 14% 43% 7%)"],
        ["#fb71f6", "color(a98-rgb 87% 44% 95%)"],
        ["#005b7c", "color(a98-rgb 9% 36% 48%)"],
        ["#e9cb87", "color(a98-rgb 88% 79% 54%)"],
        ["#004392", "color(a98-rgb 7% 27% 56%)"],
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
        ["#00000000", "color(from #bad455 a98-rgb none none none / none)"],

        // bare number
        ["#5c2ffa", "color(from #fff a98-rgb 0.325 0.198 0.962)"],
        ["#0077cb", "color(from #fff a98-rgb 0.05 0.462 0.781)"],
        //["#a767d1", "color(from #fff a98-rgb 0.593 0.403 0.804)"],
        ["#e9e5d0", "color(from #fff a98-rgb 0.906 0.897 0.814)"],
        ["#28dcdc", "color(from #fff a98-rgb 0.501 0.858 0.859)"],
        ["#ddf682", "color(from #fff a98-rgb 0.894 0.964 0.534)"],
        ["#006605", "color(from #fff a98-rgb 0.188 0.401 0.104)"],
        ["#cde8f8", "color(from #fff a98-rgb 0.83 0.906 0.969)"],
        ["#ff0a50", "color(from #fff a98-rgb 0.928 0.071 0.312)"],
        ["#6c02c6", "color(from #fff a98-rgb 0.364 0.034 0.756)"],
        ["#e66094", "color(from #fff a98-rgb 0.794 0.378 0.567)"],
        ["#d3bd00", "color(from #fff a98-rgb 0.798 0.735 0.008)"],
        ["#cf04ef", "color(from #fff a98-rgb 0.694 0.048 0.919)"],
        ["#5a2c76", "color(from #fff a98-rgb 0.319 0.188 0.452)"],
        ["#e13400", "color(from #fff a98-rgb 0.761 0.215 0.03)"],

        // percentage
        ["#0043ad", "color(from #fff a98-rgb 8% 27% 66%)"],
        ["#9e53f9", "color(from #fff a98-rgb 55% 33% 96%)"],
        ["#f68e00", "color(from #fff a98-rgb 87% 55% 4%)"],
        ["#00fdf7", "color(from #fff a98-rgb 10% 99% 97%)"],
        ["#00d09d", "color(from #fff a98-rgb 29% 81% 62%)"],
        ["#f830b2", "color(from #fff a98-rgb 84% 20% 68%)"],
        ["#ffac62", "color(from #fff a98-rgb 99% 67% 40%)"],
        ["#a8a5cc", "color(from #fff a98-rgb 65% 64% 79%)"],
        ["#ff5615", "color(from #fff a98-rgb 98% 34% 13%)"],
        //["#f9aaed", "color(from #fff a98-rgb 90% 66% 92%)"],
        ["#ff432e", "color(from #fff a98-rgb 93% 27% 20%)"],
        ["#132a7d", "color(from #fff a98-rgb 13% 18% 48%)"],
        ["#4666b6", "color(from #fff a98-rgb 32% 40% 70%)"],
        ["#c2c653", "color(from #fff a98-rgb 76% 77% 36%)"],
        ["#d288a0", "color(from #fff a98-rgb 75% 53% 62%)"],

        // keywords
        ["#bad455", "color(from #bad455 a98-rgb r g b / alpha)"],
        ["#00c2d5", "color(from #bad455 a98-rgb b r g)"],

        // calc(...)
        ["#bad455", "color(from #bad455 a98-rgb calc(r) calc(g) calc(b))"],
        ["#f05fc5", "color(from #bad455 a98-rgb calc(g) calc(b) calc(r))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
