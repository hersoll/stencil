use types::{
    colors::HexColor,
    pdf::{DocumentOptions, SolutionDecoration},
};

/// Reads the document options and writes the color macros in the Typst preamble
pub fn get_color_preamble(document_options: &DocumentOptions) -> String {
    let solution_text = &document_options.solution_text_color;
    let solution_color = match document_options.solution_decoration {
        SolutionDecoration::Fill => &document_options.solution_fill_color,
        SolutionDecoration::Border => &document_options.solution_border_color,
        SolutionDecoration::None => &HexColor::white(),
    };

    let has_color = document_options.color;
    let primary = HexColor::default_primary(has_color);
    let secondary = HexColor::default_secondary(has_color);
    let tertiary = HexColor::default_tertiary(has_color);

    format!(
        r#"
#let primary_color = rgb("{primary}")
#let solution_color = rgb("{solution_color}")
#let secondary_color = rgb("{secondary}")
#let tertiary_color = rgb("{tertiary}")
#let colored(x) = text(fill: rgb("{solution_text}"), $#x$)
#let primary(x) = text(fill: primary_color, $#x$)
#let secondary(x) = text(fill: secondary_color, $#x$)
#let tertiary(x) = text(fill: tertiary_color, $#x$)
"#
    )
}
