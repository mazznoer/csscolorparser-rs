use csscolorparser::parse;

#[test]
fn absolute() {
    #[rustfmt::skip]
    let test_data = [
        // none
        ["#00000000", "color(display-p3-linear none none none / none)"],

        // bare number
        ["#6afe00", "color(display-p3-linear 0.296 0.965 0.073)"],
        ["#ba74b0", "color(display-p3-linear 0.434 0.186 0.416)"],
        ["#3299c7", "color(display-p3-linear 0.083 0.311 0.543)"],
        ["#3ed894", "color(display-p3-linear 0.162 0.668 0.321)"],
        ["#ffdae2", "color(display-p3-linear 0.989 0.711 0.759)"],
        ["#e4d8a8", "color(display-p3-linear 0.76 0.687 0.417)"],
        ["#ff4f8e", "color(display-p3-linear 0.998 0.116 0.274)"],
        ["#c0d7b2", "color(display-p3-linear 0.556 0.675 0.462)"],
        ["#53e5ce", "color(display-p3-linear 0.211 0.762 0.623)"],
        ["#cb9cf9", "color(display-p3-linear 0.551 0.341 0.899)"],
        ["#c257d6", "color(display-p3-linear 0.462 0.111 0.63)"],
        ["#d639a7", "color(display-p3-linear 0.563 0.062 0.366)"],
        ["#c89062", "color(display-p3-linear 0.525 0.29 0.142)"],
        ["#ffc0ba", "color(display-p3-linear 0.992 0.544 0.506)"],
        ["#79bd76", "color(display-p3-linear 0.249 0.499 0.205)"],
        ["#f4b700", "color(display-p3-linear 0.829 0.488 0.024)"],
        ["#ff0052", "color(display-p3-linear 0.961 0.031 0.096)"],
        ["#9238ad", "color(display-p3-linear 0.242 0.048 0.387)"],
        ["#9f88eb", "color(display-p3-linear 0.329 0.251 0.777)"],
        ["#73b065", "color(display-p3-linear 0.219 0.428 0.152)"],
        ["#fbd422", "color(display-p3-linear 0.91 0.669 0.079)"],
        ["#661be0", "color(display-p3-linear 0.111 0.015 0.685)"],
        ["#d4eeb0", "color(display-p3-linear 0.69 0.845 0.47)"],
        ["#ca21a2", "color(display-p3-linear 0.487 0.034 0.339)"],
        ["#f964a8", "color(display-p3-linear 0.801 0.154 0.384)"],

        // percentage
        ["#da4472", "color(display-p3-linear 59% 8% 17%)"],
        ["#e22c5f", "color(display-p3-linear 63% 5% 12%)"],
        ["#b4dbe9", "color(display-p3-linear 50% 70% 80%)"],
        ["#efe700", "color(display-p3-linear 85% 80% 1%)"],
        ["#64cb76", "color(display-p3-linear 21% 58% 21%)"],
        ["#00e3f7", "color(display-p3-linear 9% 74% 90%)"],
        ["#43fdf6", "color(display-p3-linear 22% 95% 91%)"],
        ["#a9ef00", "color(display-p3-linear 48% 85% 5%)"],
        ["#b294b3", "color(display-p3-linear 42% 30% 44%)"],
        ["#f0c2aa", "color(display-p3-linear 81% 55% 42%)"],
        ["#7ddfef", "color(display-p3-linear 30% 72% 84%)"],
        ["#aae6ba", "color(display-p3-linear 47% 78% 51%)"],
        ["#c505b5", "color(display-p3-linear 46% 2% 43%)"],
        ["#ff9fba", "color(display-p3-linear 93% 37% 49%)"],
        ["#d9e681", "color(display-p3-linear 71% 79% 27%)"],
        ["#5d82a3", "color(display-p3-linear 13% 22% 35%)"],
        ["#e6f9d4", "color(display-p3-linear 82% 94% 68%)"],
        //["#19ff90", "color(display-p3-linear 19% 99% 33%)"],
        //["#dce899", "color(display-p3-linear 73% 80% 36%)"],
        ["#ff5bd1", "color(display-p3-linear 96% 14% 61%)"],
        ["#edfde5", "color(display-p3-linear 87% 98% 80%)"],
        ["#a8f922", "color(display-p3-linear 49% 93% 9%)"],
        ["#a86de3", "color(display-p3-linear 35% 16% 72%)"],
        ["#d9fafe", "color(display-p3-linear 74% 95% 98%)"],
        ["#bb4231", "color(display-p3-linear 42% 7% 4%)"],
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
        ["#00000000", "color(from #bad455 display-p3-linear none none none / none)"],

        // bare number
        ["#00e717", "color(from #fff display-p3-linear 0.106 0.771 0.065)"],
        //["#78abb5", "color(from #fff display-p3-linear 0.228 0.399 0.452)"],
        ["#82fde7", "color(from #fff display-p3-linear 0.358 0.958 0.8)"],
        ["#e4fafc", "color(from #fff display-p3-linear 0.806 0.947 0.966)"],
        ["#fc00f1", "color(from #fff display-p3-linear 0.799 0.015 0.816)"],
        ["#cfa0be", "color(from #fff display-p3-linear 0.576 0.36 0.505)"],
        ["#7d6be0", "color(from #fff display-p3-linear 0.195 0.15 0.695)"],
        ["#f9eab8", "color(from #fff display-p3-linear 0.922 0.823 0.512)"],
        ["#b67cbb", "color(from #fff display-p3-linear 0.42 0.209 0.474)"],
        ["#d9b64b", "color(from #fff display-p3-linear 0.653 0.476 0.11)"],
        ["#91fdb3", "color(from #fff display-p3-linear 0.406 0.959 0.486)"],
        ["#79cf7c", "color(from #fff display-p3-linear 0.268 0.607 0.232)"],
        ["#eaf600", "color(from #fff display-p3-linear 0.84 0.917 0.039)"],
        ["#d45124", "color(from #fff display-p3-linear 0.556 0.102 0.033)"],
        ["#78bfff", "color(from #fff display-p3-linear 0.248 0.509 0.979)"],

        // percentage
        ["#bb9362", "color(from #fff display-p3-linear 46% 30% 14%)"],
        ["#f0fd67", "color(from #fff display-p3-linear 89% 98% 21%)"],
        //["#5870ff", "color(from #fff display-p3-linear 11% 16% 92%)"],
        ["#e2fedb", "color(from #fff display-p3-linear 80% 98% 73%)"],
        ["#78d297", "color(from #fff display-p3-linear 27% 63% 33%)"],
        ["#97317f", "color(from #fff display-p3-linear 26% 4% 20%)"],
        ["#dc79d3", "color(from #fff display-p3-linear 62% 21% 62%)"],
        ["#fedb75", "color(from #fff display-p3-linear 94% 72% 23%)"],
        ["#0093ff", "color(from #fff display-p3-linear 5% 28% 96%)"],
        ["#e2f562", "color(from #fff display-p3-linear 79% 91% 19%)"],
        ["#00e5e7", "color(from #fff display-p3-linear 2% 75% 78%)"],
        ["#75ad77", "color(from #fff display-p3-linear 22% 41% 20%)"],
        ["#d2c661", "color(from #fff display-p3-linear 63% 57% 16%)"],
        //["#3cee52", "color(from #fff display-p3-linear 19% 83% 14%)"],
        //["#ff24ed", "color(from #fff display-p3-linear 83% 5% 79%)"],

        // keywords
        ["#bad455", "color(from #bad455 display-p3-linear r g b)"],
        ["#41c2d6", "color(from #bad455 display-p3-linear b r g)"],

        // calc(...)
        ["#bad455", "color(from #bad455 display-p3-linear calc(r) calc(g) calc(b))"],
        ["#e360c3", "color(from #bad455 display-p3-linear calc(g) calc(b) calc(r))"],
        ["#bad455", "color(from #fff display-p3-linear calc((7.92048 - 6.878748) / (5.591231 - 3.591231)) calc(64.72245 / 10 - 58.19487 / 10) calc(12.179576 / 2 - (3.572552 + 2.378498)))"],
        ["#bad455", "color(from #fff display-p3-linear calc(0.383296 / 2 + 0.658436 / 2) calc((0.34083 + 0.15917) * (8.363013 - 7.057497)) calc(0.009775 + 0.128963))"],
        ["#bad455", "color(from #fff display-p3-linear calc(0.260433 * 2) calc(6.850732 - 6.733431 + 2.141828 / 4) calc((5.490034 - 5.07382) / (12.951016 - 9.951016)))"],
        ["#bad455", "color(from #fff display-p3-linear calc(8.854462 - 8.409099 + 0.151006 / 2) calc((2.877809 - 2.627809) * (4.785033 - 2.174001)) calc(0.002561 + 0.005806 + 0.25 * 0.521484))"],
        ["#bad455", "color(from #fff display-p3-linear calc(0.958186 * 5 - (2.610794 + 1.65927)) calc((0.461535 + 0.843981) * 0.5) calc(0.259472 / 4 + 0.25 * 0.29548))"],
        ["#bad455", "color(from #8fc381 display-p3-linear calc((2.406361 - r) * 0.5 * 0.5) calc(g + 2.840318 - 2.724236) calc((b - 0.174677) * (0.392545 + 1.607455)))"],
        ["#bad455", "color(from #e2fda1 display-p3-linear calc(5.896649 - r - 4.575692) calc(0.208307 + g - 0.530344) calc((b - 0.339177) / 0.5))"],
        ["#bad455", "color(from #223fdf display-p3-linear calc(3.6624538 - (3.1635737 - r)) calc(g + 2.4167288 * 0.25) calc(-0.644272 + b + 3.198829 - 3.091482))"],
        ["#bad455", "color(from #acf6a3 display-p3-linear calc(r + 9.880283 - (2.9682 + 6.894296)) calc(6.003656 - (6.255498 - g)) calc(b - 0.324963 + 0.022972 + 0.033545))"],
        ["#bad455", "color(from #cda4dd display-p3-linear calc(0.25 * (r + 1.515345)) calc((g + 0.926383) * 1.5 / 3) calc(4.372937 + 2.718273 - (6.256911 + b)))"],
    ];

    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
