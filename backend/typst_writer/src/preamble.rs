pub static PREAMBLE_STR: &str = r##"
#import "/preamble.typ": *
#import "@preview/equate:0.3.2": equate, share-align
#import "@preview/zero:0.5.0": num, set-num
#import "@preview/cetz:0.4.2"
#import "@preview/cetz-plot:0.1.3": plot
#show: equate.with(debug: false)
#show math.equation.where(block: false): box
#set enum(spacing: 1.33em)
#set enum(numbering: it => {
  item-counter.step()
  context box(width: 1.35em, text(weight: "bold")[#item-counter.display())])
})
#set-num(decimal-separator: ",")
#let item(content) = block(breakable: false, content)
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

    let color-operations = operations.map(op => if op != $$ { colored(op) } else { op })

    share-align({
      grid(
        columns: (max-eq-width, auto),
        inset: (top: 0.2em, rest: 0.5em),
        align: (left, horizon + left),
        grid.vline(x: 1, stroke: (paint: line_color, thickness: 0.5pt)),
        ..equations.zip(color-operations).flatten(),
      )
    })
  }
}

"##;
