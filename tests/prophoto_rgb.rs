use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(prophoto-rgb none none none / none)"],

        // bare number
        ["#00ca9f", "color(prophoto-rgb 0.148 0.669 0.57)"],
        ["#d200aa", "color(prophoto-rgb 0.588 0.163 0.56)"],
        ["#00cec3", "color(prophoto-rgb 0.435 0.712 0.713)"],
        ["#95df00", "color(prophoto-rgb 0.594 0.8 0.132)"],
        ["#001fee", "color(prophoto-rgb 0.152 0.111 0.847)"],
        ["#d96200", "color(prophoto-rgb 0.604 0.378 0.036)"],
        ["#003494", "color(prophoto-rgb 0.017 0.138 0.473)"],
        ["#ab75ad", "color(prophoto-rgb 0.544 0.417 0.593)"],
        ["#ffc89a", "color(prophoto-rgb 0.898 0.768 0.573)"],
        ["#00b6a4", "color(prophoto-rgb 0.092 0.585 0.574)"],
        ["#ff0098", "color(prophoto-rgb 0.764 0.233 0.497)"],
        ["#005ad7", "color(prophoto-rgb 0.114 0.259 0.75)"],
        ["#00c700", "color(prophoto-rgb 0.262 0.665 0.173)"],
        ["#00ff00", "color(prophoto-rgb 0.193 0.949 0.16)"],
        ["#ff1090", "color(prophoto-rgb 0.984 0.392 0.484)"],
        ["#ec00e4", "color(prophoto-rgb 0.711 0.242 0.808)"],
        ["#cec2e9", "color(prophoto-rgb 0.767 0.721 0.869)"],
        ["#dfceff", "color(prophoto-rgb 0.846 0.781 0.988)"],
        ["#98c06e", "color(prophoto-rgb 0.57 0.679 0.412)"],
        ["#ecfe00", "color(prophoto-rgb 0.852 0.967 0.026)"],
        ["#00bced", "color(prophoto-rgb 0.149 0.614 0.874)"],
        ["#009d00", "color(prophoto-rgb 0.122 0.492 0.15)"],
        ["#4b6eb9", "color(prophoto-rgb 0.359 0.359 0.635)"],
        ["#00ff00", "color(prophoto-rgb 0.155 0.995 0.145)"],
        ["#ff85ff", "color(prophoto-rgb 0.956 0.571 0.988)"],

        // percentage
        ["#008100", "color(prophoto-rgb 13% 39% 11%)"],
        //["#a20011", "color(prophoto-rgb 40% 15% 8%)"],
        ["#ff6449", "color(prophoto-rgb 96% 49% 28%)"],
        ["#dd95da", "color(prophoto-rgb 74% 56% 79%)"],
        ["#c7a831", "color(prophoto-rgb 63% 60% 25%)"],
        ["#4b38bd", "color(prophoto-rgb 31% 20% 64%)"],
        ["#00e2f4", "color(prophoto-rgb 27% 78% 92%)"],
        ["#00bb00", "color(prophoto-rgb 30% 62% 6%)"],
        ["#e0daae", "color(prophoto-rgb 81% 82% 65%)"],
        ["#e90092", "color(prophoto-rgb 65% 20% 47%)"],
        ["#7eb7c0", "color(prophoto-rgb 55% 64% 69%)"],
        ["#00d3d4", "color(prophoto-rgb 33% 72% 78%)"],
        ["#00fff2", "color(prophoto-rgb 9% 91% 93%)"],
        ["#022422", "color(prophoto-rgb 7% 10% 10%)"],
        ["#bf6474", "color(prophoto-rgb 55% 37% 38%)"],
        ["#e26555", "color(prophoto-rgb 65% 40% 29%)"],
        ["#003084", "color(prophoto-rgb 5% 13% 41%)"],
        ["#6159e8", "color(prophoto-rgb 42% 31% 83%)"],
        ["#d7da66", "color(prophoto-rgb 76% 81% 42%)"],
        ["#55cc48", "color(prophoto-rgb 47% 71% 32%)"],
        ["#5c8e00", "color(prophoto-rgb 34% 46% 11%)"],
        ["#fff405", "color(prophoto-rgb 97% 95% 32%)"],
        ["#00ff00", "color(prophoto-rgb 6% 96% 22%)"],
        ["#ff0065", "color(prophoto-rgb 98% 33% 33%)"],
        ["#518c3c", "color(prophoto-rgb 33% 45% 23%)"],
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
        ["#00000000", "color(from #bad455 prophoto-rgb none none none / none)"],

        // bare number
        ["#0000ee", "color(from #fff prophoto-rgb 0.264 0.028 0.842)"],
        ["#ffd200", "color(from #fff prophoto-rgb 0.84 0.794 0.096)"],
        ["#00bab1", "color(from #fff prophoto-rgb 0.242 0.612 0.627)"],
        ["#ff5d00", "color(from #fff prophoto-rgb 0.948 0.471 0.165)"],
        ["#006d00", "color(from #fff prophoto-rgb 0.066 0.317 0.068)"],
        ["#87b600", "color(from #fff prophoto-rgb 0.488 0.626 0.069)"],
        ["#e4d8c2", "color(from #fff prophoto-rgb 0.829 0.816 0.724)"],
        ["#004db1", "color(from #fff prophoto-rgb 0.048 0.212 0.59)"],
        ["#00fffd", "color(from #fff prophoto-rgb 0.068 0.962 0.981)"],
        ["#00dc00", "color(from #fff prophoto-rgb 0.401 0.766 0.042)"],
        ["#ffc000", "color(from #fff prophoto-rgb 0.885 0.738 0.145)"],
        ["#b5bc25", "color(from #fff prophoto-rgb 0.611 0.668 0.251)"],
        ["#ff8c00", "color(from #fff prophoto-rgb 0.857 0.56 0.136)"],
        //["#c2ffda", "color(from #fff prophoto-rgb 0.835 0.984 0.842)"],
        ["#5156c7", "color(from #fff prophoto-rgb 0.354 0.286 0.684)"],

        // percentage
        ["#8700ff", "color(from #fff prophoto-rgb 48% 17% 93%)"],
        ["#004f07", "color(from #fff prophoto-rgb 8% 22% 8%)"],
        ["#00a3e4", "color(from #fff prophoto-rgb 44% 55% 83%)"],
        ["#cd2500", "color(from #fff prophoto-rgb 54% 24% 8%)"],
        ["#ff00f9", "color(from #fff prophoto-rgb 92% 20% 90%)"],
        ["#ffda6d", "color(from #fff prophoto-rgb 98% 85% 45%)"],
        ["#fc8100", "color(from #fff prophoto-rgb 74% 50% 8%)"],
        ["#ffd700", "color(from #fff prophoto-rgb 93% 83% 7%)"],
        //["#fd000a", "color(from #fff prophoto-rgb 69% 23% 10%)"],
        ["#d6002e", "color(from #fff prophoto-rgb 56% 19% 15%)"],
        ["#dceea7", "color(from #fff prophoto-rgb 83% 90% 64%)"],
        ["#00b578", "color(from #fff prophoto-rgb 5% 58% 42%)"],
        ["#0018ff", "color(from #fff prophoto-rgb 15% 10% 94%)"],
        ["#acda00", "color(from #fff prophoto-rgb 64% 79% 22%)"],
        ["#76004d", "color(from #fff prophoto-rgb 28% 6% 22%)"],

        // keywords
        ["#bad455", "color(from #bad455 prophoto-rgb r g b / alpha)"],
        ["#ff44c2", "color(from #bad455 prophoto-rgb g b r)"],

        // calc(...)
        ["#bad455", "color(from #bad455 prophoto-rgb calc(r) calc(g) calc(b) / calc(alpha))"],
        ["#00c6d3", "color(from #bad455 prophoto-rgb calc(b) calc(r) calc(g))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
