pub static PREAMBLE_STR: &str = r#"
#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)


//Colors
#let gray(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)
#let linecolor = color.linear-rgb(20%, 20%, 20%)

//Enum settings
#let item-counter = counter("item-counter")
#set enum(numbering: it => box(width: 1em, text(weight: "bold")[#it)]))
#show enum: it => {
  if it.start != 0 { return it }
  let args = it.fields()
  let items = args.remove("children")
  context enum(..args, start: item-counter.get().first() + 1, ..items)
  item-counter.update(i => i + it.children.len())
}

//Equation solution template
#let equation-solution(equations, operations) = {
  context {
    let max-eq-width = 0pt
    let max-op-width = 0pt

    for eq in equations {
      let size = measure(eq)
      if size.width > max-eq-width {
        max-eq-width = size.width
      }
    }

    let gray-operations = operations.map(op => if op != $$ { gray(op) } else { op })

    share-align({
      grid(
        columns: (max-eq-width, auto),
        inset: 5pt,
        align: (left, horizon + left),
        grid.vline(x: 1, stroke: (paint: linecolor, thickness: 0.5pt)),
        ..equations.zip(gray-operations).flatten(),
      )
    })
  }
}
"#;
