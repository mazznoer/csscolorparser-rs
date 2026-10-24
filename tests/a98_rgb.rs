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
