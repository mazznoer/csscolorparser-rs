// Usage:
// cargo r --example cli -- [color]..

use csscolorparser::parse;

fn main() {
    for arg in std::env::args().skip(1) {
        println!("{arg:?}");
        match parse(&arg) {
            Ok(c) => {
                let [r, g, b, _] = c.to_rgba8();
                let name = if let Some(s) = c.name() { s } else { "-" };
                println!("    \x1B[48;2;{r};{g};{b}m        \x1B[49m");
                println!("    {}", c);
                println!("    {}", c.to_css_rgb());
                println!("    {}", c.to_css_hwb());
                println!("    {}", c.to_css_hsl());
                println!("    {}", c.to_css_lab());
                println!("    {}", c.to_css_lch());
                println!("    {}", c.to_css_oklab());
                println!("    {}", c.to_css_oklch());
                println!("    name {}", name);
            }
            Err(e) => println!("    {e}"),
        }
    }
}
