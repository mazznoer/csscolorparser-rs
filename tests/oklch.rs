use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        "oklch(0.284 0.132 0)",
        "oklch(0.314 0.136 17)",
        "oklch(0.935 0.398 35)",
        "oklch(0.729 0.175 53)",
        "oklch(0.157 0.29 71)",
        "oklch(0.266 0.365 89)",
        "oklch(0.12 0.225 107)",
        "oklch(0.532 0.274 125)",
        "oklch(0.571 0.201 143)",
        "oklch(0.948 0.217 161)",
        "oklch(0.501 0.2 179)",
        "oklch(0.184 0.308 197)",
        "oklch(0.308 0.273 215)",
        "oklch(0.874 0.143 233)",
        "oklch(0.544 0.186 251)",
        "oklch(0.144 0.255 269)",
        "oklch(0.997 0.327 287)",
        "oklch(0.544 0.22 305)",
        "oklch(0.578 0.203 323)",
        "oklch(0.819 0.343 341)",
        "oklch(0.497 0.188 359)",
    ];
    for s in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        let c = c.unwrap();
        assert_eq!(s, c.to_css_oklch().to_string(), "{s:?}");
    }
}

#[test]
fn relative() {
    let test_data = [
        // none
        ["#000000", "oklch(from #bad455 none none none)"],
        ["#bad45500", "oklch(from #bad455 l c h / none)"],
        // bare number
        ["#ff80be", "oklch(from #fff 0.818 0.208 0)"],
        ["#610000", "oklch(from #fff 0.252 0.193 25)"],
        ["#ff0000", "oklch(from #fff 0.75 0.38 51)"],
        //["#ffb532", "oklch(from #fff 0.822 0.159 76)"],
        ["#806400", "oklch(from #fff 0.509 0.209 102)"],
        ["#beff49", "oklch(from #fff 0.931 0.217 128)"],
        ["#13cc6f", "oklch(from #fff 0.743 0.188 153)"],
        ["#00432c", "oklch(from #fff 0.286 0.207 179)"],
        ["#009eac", "oklch(from #fff 0.634 0.117 205)"],
        ["#00dcff", "oklch(from #fff 0.824 0.262 230)"],
        ["#220097", "oklch(from #fff 0.182 0.305 256)"],
        ["#c1c4ff", "oklch(from #fff 0.847 0.111 282)"],
        ["#a848ef", "oklch(from #fff 0.607 0.24 307)"],
        ["#ff5fff", "oklch(from #fff 0.778 0.271 333)"],
        ["#ffd9f8", "oklch(from #fff 0.985 0.116 359)"],
        // percentage
        ["#004439", "oklch(from #fff 34% 18% 179)"],
        ["#200059", "oklch(from #fff 9% 69% 299)"],
        ["#004bb1", "oklch(from #fff 43% 47% 251)"],
        ["#ffb6ca", "oklch(from #fff 88% 30% 7)"],
        ["#e7bb4c", "oklch(from #fff 81% 34% 87)"],
        ["#7e7572", "oklch(from #fff 57% 3% 43)"],
        ["#822b31", "oklch(from #fff 42% 30% 20)"],
        ["#920000", "oklch(from #fff 39% 49% 33)"],
        ["#5e33d2", "oklch(from #fff 48% 56% 287)"],
        ["#2f556b", "oklch(from #fff 43% 14% 236)"],
        ["#0095d8", "oklch(from #fff 56% 79% 210)"],
        ["#1d0000", "oklch(from #fff 12% 22% 17)"],
        ["#d4e200", "oklch(from #fff 87% 56% 114)"],
        ["#00ffab", "oklch(from #fff 88% 61% 164)"],
        ["#33424a", "oklch(from #fff 37% 6% 231)"],
        // keywords
        ["#bad455", "oklch(from #bad455 l c h)"],
        ["#bad455", "oklch(from #bad455 l c h / alpha)"],
        // calc(...)
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
        "oklch(0 0 0%)",
        "oklch(from #bad455 l c 0%)",
    ];
    for s in test_data {
        assert!(parse(s).is_err(), "{s:?}");
    }
}
