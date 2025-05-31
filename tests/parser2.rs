use csscolorparser::parse;

#[test]
fn rgb() {
    let test_data = [
        "rgb(71 175 99)",
        "rgb(170 203 72)",
        "rgb(45 232 237)",
        "rgb(119 1 124)",
        "rgb(243 93 86)",
        "rgb(223 25 119)",
        "rgb(6 44 133)",
        "rgb(167 240 237)",
        "rgb(97 71 129)",
        "rgb(125 68 93)",
        "rgb(139 187 62)",
        "rgb(100 51 80)",
        "rgb(27 249 123)",
        "rgb(230 63 99)",
        "rgb(241 34 4)",
        "rgb(149 222 185)",
        "rgb(3 129 213)",
        "rgb(88 220 108)",
        "rgb(199 169 6)",
        "rgb(54 70 163)",
        "rgb(90 42 106)",
    ];
    for s in test_data {
        let c = parse(s).unwrap();
        assert_eq!(s, c.to_css_rgb());
    }
}

#[test]
fn hsl() {
    let test_data = [
    /*
        "hsl(0 48% 83%)",
        "hsl(17 73% 13%)",
        "hsl(35 40% 84%)",
        "hsl(53 88% 21%)",
        "hsl(71 11% 45%)",
        "hsl(89 12% 89%)",
        "hsl(107 49% 68%)",
        "hsl(125 96% 72%)",
        "hsl(143 15% 92%)",
        "hsl(161 80% 93%)",
        "hsl(179 45% 76%)",
        "hsl(197 99% 84%)",
        "hsl(215 33% 15%)",
        "hsl(233 69% 59%)",
        "hsl(251 34% 46%)",
        "hsl(269 43% 18%)",
        "hsl(287 89% 69%)",
        "hsl(305 87% 36%)",
        "hsl(323 97% 26%)",
        "hsl(341 61% 66%)",
        "hsl(359 15% 74%)",
    */
    ];
    for s in test_data {
        let c = parse(s).unwrap();
        assert_eq!(s, c.to_css_hsl());
    }
}

#[test]
fn hwb() {
    let test_data = [
    /*
        "hwb(0 87% 0%)",
        "hwb(17 0% 23%)",
        "hwb(35 0% 7%)",
        "hwb(53 66% 0%)",
        "hwb(71 0% 66%)",
        "hwb(89 22% 0%)",
        "hwb(107 0% 2%)",
        "hwb(125 51% 0%)",
        "hwb(143 10% 0%)",
        "hwb(161 0% 76%)",
        "hwb(179 72% 0%)",
        "hwb(197 0% 60%)",
        "hwb(215 0% 39%)",
        "hwb(233 0% 18%)",
        "hwb(251 0% 3%)",
        "hwb(269 57% 0%)",
        "hwb(287 21% 0%)",
        "hwb(305 15% 0%)",
        "hwb(323 55% 0%)",
        "hwb(341 0% 72%)",
        "hwb(359 0% 2%)",
    */
    ];
    for s in test_data {
        let c = parse(s).unwrap();
        assert_eq!(s, c.to_css_hwb());
    }
}

#[test]
fn oklab() {
    let test_data = [
        "oklab(0.623 0.019 -0.359)",
        "oklab(0.362 -0.314 -0.035)",
        "oklab(0.804 0.166 -0.072)",
        "oklab(0.832 0.089 0.265)",
        "oklab(0.681 0.038 -0.3)",
        "oklab(0.117 -0.192 0.24)",
        "oklab(0.651 -0.241 -0.158)",
        "oklab(0.421 -0.248 0.053)",
        "oklab(0.923 -0.119 -0.288)",
        "oklab(0.811 -0.295 0.347)",
        "oklab(0.485 -0.368 0.066)",
        "oklab(0.905 0.13 -0.163)",
        "oklab(0.778 -0.001 0.4)",
        "oklab(0.672 0.136 -0.03)",
        "oklab(0.926 0.281 0.279)",
        "oklab(0.247 0.155 0.379)",
        "oklab(0.503 0.042 0.202)",
        "oklab(0.792 -0.34 -0.372)",
        "oklab(0.877 -0.13 0.222)",
        "oklab(0.898 -0.068 -0.239)",
        "oklab(0.725 -0.343 -0.352)",
    ];
    for s in test_data {
        let c = parse(s).unwrap();
        assert_eq!(s, c.to_css_oklab());
    }
}

#[test]
fn oklch() {
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
        let c = parse(s).unwrap();
        assert_eq!(s, c.to_css_oklch());
    }
}
