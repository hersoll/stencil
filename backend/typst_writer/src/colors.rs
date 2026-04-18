use std::fmt::Display;

pub static GRAPHING_COLORS: [&'static str; 3] = ["primary", "secondary", "tertiary"];

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%, {}%, {}%", self.r, self.g, self.b)
    }
}

pub fn get_color_preamble(has_color: bool) -> String {
    let colored: Color;
    // Graphing colors
    let primary: Color;
    let secondary: Color;
    let tertiary: Color;
    if has_color {
        colored = Color::new(22, 10, 33); // Purple
        primary = Color::new(9, 3, 18); // Dark purple
        secondary = colored.clone();
        tertiary = Color::new(30, 23, 39); // Light purple
    } else {
        colored = Color::new(10, 10, 10); // Gray
        primary = Color::new(0, 0, 0); // Black
        secondary = Color::new(8, 8, 8); // Gray?
        tertiary = Color::new(16, 16, 16); // Grayer?
    };

    format!(
        "
#let primary_color = color.linear-rgb({primary})
#let secondary_color = color.linear-rgb({secondary})
#let tertiary_color = color.linear-rgb({tertiary})
#let colored(x) = text(fill: color.linear-rgb({colored}), $#x$)
#let primary(x) = text(fill: primary_color, $#x$)
#let secondary(x) = text(fill: secondary_color, $#x$)
#let tertiary(x) = text(fill: tertiary_color, $#x$)
"
    )
}
