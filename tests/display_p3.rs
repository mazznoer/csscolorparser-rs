use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(display-p3 none none none / none)"],

        // bare number
        ["#008840", "color(display-p3 0.007 0.525 0.279)"],
        ["#5e69ac", "color(display-p3 0.377 0.411 0.656)"],
        ["#a28e7e", "color(display-p3 0.622 0.56 0.503)"],
        ["#00908c", "color(display-p3 0.167 0.556 0.544)"],
        ["#009be7", "color(display-p3 0.142 0.596 0.882)"],
        ["#73aae2", "color(display-p3 0.497 0.659 0.868)"],
        //["#6621e7", "color(display-p3 0.367 0.146 0.871)"],
        ["#374f00", "color(display-p3 0.236 0.306 0.023)"],
        ["#a949ff", "color(display-p3 0.616 0.309 0.974)"],
        ["#70b5a1", "color(display-p3 0.5 0.701 0.635)"],
        ["#ffe100", "color(display-p3 0.998 0.886 0.243)"],
        ["#be16c1", "color(display-p3 0.684 0.171 0.733)"],
        ["#a325e2", "color(display-p3 0.588 0.189 0.854)"],
        //["#da678a", "color(display-p3 0.8 0.43 0.541)"],
        ["#4b091c", "color(display-p3 0.267 0.06 0.112)"],
        ["#ffb7bf", "color(display-p3 0.972 0.73 0.754)"],
        ["#476335", "color(display-p3 0.301 0.386 0.229)"],
        ["#919d43", "color(display-p3 0.578 0.614 0.315)"],
        ["#790d3a", "color(display-p3 0.434 0.102 0.227)"],
        ["#8c6aad", "color(display-p3 0.528 0.421 0.66)"],
        ["#a000d9", "color(display-p3 0.572 0.085 0.82)"],
        ["#d2569f", "color(display-p3 0.767 0.369 0.614)"],
        ["#ffa36a", "color(display-p3 0.995 0.659 0.459)"],
        //["#459a77", "color(display-p3 0.361 0.597 0.476)"],
        ["#826f43", "color(display-p3 0.498 0.437 0.286)"],

        // percentage
        ["#9b5300", "color(display-p3 57% 34% 6%)"],
        //["#e3e000", "color(display-p3 89% 88% 17%)"],
        ["#f2d5cd", "color(display-p3 93% 84% 81%)"],
        ["#00caa7", "color(display-p3 20% 78% 66%)"],
        ["#1a36f7", "color(display-p3 13% 21% 93%)"],
        ["#ceb400", "color(display-p3 79% 71% 20%)"],
        ["#7fa7ff", "color(display-p3 53% 65% 98%)"],
        ["#3021ff", "color(display-p3 18% 13% 98%)"],
        //["#65a2ad", "color(display-p3 45% 63% 67%)"],
        ["#3ce19b", "color(display-p3 45% 87% 63%)"],
        ["#00a66f", "color(display-p3 5% 64% 45%)"],
        //["#c9aabb", "color(display-p3 77% 67% 73%)"],
        ["#d02600", "color(display-p3 75% 22% 0%)"],
        ["#611939", "color(display-p3 35% 12% 22%)"],
        ["#2e7797", "color(display-p3 26% 46% 58%)"],
        ["#837fca", "color(display-p3 51% 50% 77%)"],
        ["#0e4900", "color(display-p3 13% 28% 5%)"],
        ["#ff0000", "color(display-p3 99% 11% 8%)"],
        ["#0097be", "color(display-p3 0% 58% 73%)"],
        ["#f200db", "color(display-p3 87% 9% 83%)"],
        ["#a85a5f", "color(display-p3 62% 37% 38%)"],
        ["#a5da00", "color(display-p3 69% 85% 7%)"],
        ["#953aab", "color(display-p3 54% 25% 65%)"],
        ["#ea00a8", "color(display-p3 84% 1% 64%)"],
        ["#4bbd7d", "color(display-p3 42% 73% 51%)"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
