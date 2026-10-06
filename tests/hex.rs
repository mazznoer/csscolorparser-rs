use csscolorparser::parse;

#[test]
fn in_out() {
    let test_data = [
        "#71fe15",
        "#d6e3c9",
        "#2a7719",
        "#b53717",
        "#5b0b8d",
        "#aff632",
        "#65ec8d",
        "#d35493",
        "#289e5f",
        "#b46152",
        "#e0afee",
        "#ac2be4",
        "#233490",
        "#1afbc5",
        "#e41755",
        "#e052ee",
        "#4d1b5e",
        "#230cde",
        "#f8a243",
        "#a130d1",
        "#b38373",
        // alpha
        "#6b9fa203",
        "#0e5e0be6",
        "#84f9a716",
        "#48651550",
        "#1adc2cf4",
        "#c191a31c",
        "#a25518c5",
        "#cb33f2c9",
        "#89b21d36",
        "#cbb97f3e",
    ];
    for s in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(s, c.unwrap().to_string(), "{s:?}");
    }
}

#[test]
fn shorthand() {
    let test_data = [
        ["#6611aa", "#61a"],
        ["#44cccc", "#4cc"],
        ["#22ff44", "#2f4"],
        ["#ff4499", "#f49"],
        ["#44eebb", "#4eb"],
        ["#222277", "#227"],
        ["#66bb77", "#6b7"],
        ["#11dd77", "#1d7"],
        ["#555588", "#558"],
        ["#8899bb", "#89b"],
        ["#3366bb", "#36b"],
        ["#ddee55", "#de5"],
        ["#11dd88", "#1d8"],
        ["#ff9999", "#f99"],
        ["#bbff99", "#bf9"],
        // alpha
        ["#99aa00", "#9a0f"],
        ["#003366", "#036f"],
        ["#cc11bb55", "#c1b5"],
        ["#aa22ff77", "#a2f7"],
        ["#9900ee77", "#90e7"],
        ["#eebb2288", "#eb28"],
        ["#44bb99cc", "#4b9c"],
        ["#ee8800bb", "#e80b"],
        ["#dd66ffaa", "#d6fa"],
        ["#99cc1100", "#9c10"],
        ["#ff223366", "#f236"],
        ["#99660099", "#9609"],
        ["#88ee7722", "#8e72"],
        ["#0099ff55", "#09f5"],
        ["#44553399", "#4539"],
    ];
    for [hex, s] in test_data {
        let c = parse(s);
        assert!(c.is_ok(), "{s:?}");
        assert_eq!(hex, c.unwrap().to_string(), "{s:?}");
    }
}
