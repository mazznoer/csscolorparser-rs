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
