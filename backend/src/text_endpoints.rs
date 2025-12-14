pub async fn welcome() -> String {
    String::from(
        "Welcome to the Stencil API! \n
Endpoints (with base /api):\n
/pdf/example - Generate an example stencil
/pdf - Generate a custom stencil (GET request with JSON body required)
/help - See the schema for custom stencil requests",
    )
}

pub async fn help() -> String {
    String::from(
        "To properly request a custom stencil, a JSON body of the following format must be attached:\n",
    ) + &get_http_schema()
}
pub fn get_http_schema() -> String {
    String::from(
        r#"
{
  sets: [
    {
      topics: number[],
      exclusions: number[] (optional),
      starting_difficulty: "Intro" | "Easy" | "Medium" | "Hard",
      ending_difficulty:   "Intro" | "Easy" | "Medium" | "Hard",
      n: number
    }
  ],

  (optional)
  document_options: {
    font_size: number,
    title: string,
    answer_columns: number,
    lang: "Sv" | "En",
    write_solutions: "First" | "All" | "None",
    color: boolean,
    paper_size: "A4" | "A5",
    x_margin: number,
    y_margin: number,
    max_prefix_group: number
  }
}
"#,
    )
}
