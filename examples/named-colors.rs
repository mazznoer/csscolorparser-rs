use csscolorparser::NAMED_COLORS;

fn main() {
    for (name, rgb) in &NAMED_COLORS {
        let [r, g, b] = rgb;
        println!("\x1B[48;2;{r};{g};{b}m    \x1B[49m {name} {rgb:?}");
    }
}
