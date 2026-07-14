static TEXT_HIGHLIGHT_COLORED: &str = "#8A6AA8";
static TEXT_HIGHLIGHT_BW: &str = "#949494";
static SOLUTION_FILL_COLORED: &str = "#F7F6E9";
static SOLUTION_FILL_BW: &str = "#F9F9F9";
static PRIMARY_COLOR_COLORED: &str = "#B79DCF";
static PRIMARY_COLOR_BW: &str = "#222222";
static SECONDARY_COLOR_COLORED: &str = "#B79DCF";
static SECONDARY_COLOR_BW: &str = "#222222";
static TERTIARY_COLOR_COLORED: &str = "#B79DCF";
static TERTIARY_COLOR_BW: &str = "#222222";

pub fn get_color_preamble(has_color: bool) -> String {
    let colored;
    let solution_background;
    let primary;
    let secondary;
    let tertiary;
    if has_color {
        colored = TEXT_HIGHLIGHT_COLORED;
        solution_background = SOLUTION_FILL_COLORED;
        primary = PRIMARY_COLOR_COLORED;
        secondary = SECONDARY_COLOR_COLORED;
        tertiary = TERTIARY_COLOR_COLORED;
    } else {
        colored = TEXT_HIGHLIGHT_BW;
        solution_background = SOLUTION_FILL_BW;
        primary = PRIMARY_COLOR_BW;
        secondary = SECONDARY_COLOR_BW;
        tertiary = TERTIARY_COLOR_BW;
    };

    format!(
        r#"
#let primary_color = rgb("{primary}")
#let solution_color = rgb("{solution_background}")
#let secondary_color = rgb("{secondary}")
#let tertiary_color = rgb("{tertiary}")
#let colored(x) = text(fill: rgb("{colored}"), $#x$)
#let primary(x) = text(fill: primary_color, $#x$)
#let secondary(x) = text(fill: secondary_color, $#x$)
#let tertiary(x) = text(fill: tertiary_color, $#x$)
"#
    )
}
