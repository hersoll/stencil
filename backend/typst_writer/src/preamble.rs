pub static PREAMBLE_STR: &str = r##"
#import "/preamble.typ": *
#import "@preview/equate:0.3.2": equate, share-align
#import "@preview/zero:0.5.0": num, set-num, set-group
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
#set-group(size: 3, threshold: (integer: 4, fractional: 6))
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

    for op in operations {
      if op != $$ {
        let size = measure(op)
        if size.width > max-op-width {
          max-op-width = size.width
        }
      }
    }

    layout(available => {
      let inset = 0.5em
      let natural-width = max-eq-width + max-op-width + inset

      let factor = calc.min(
        1,
        available.width.pt() / natural-width.to-absolute().pt(),
      ) * 100%

      let color-operations = operations.map(op => if op != $$ { colored(op) } else { op })

      align(center)[
        #scale(factor, reflow: true)[
          #share-align({
            grid(
              columns: (max-eq-width, auto),
              inset: (top: 0.2em, rest: inset),
              align: (left, horizon + left),
              grid.vline(
                x: 1,
                stroke: (
                  paint: line_color,
                  thickness: 0.5pt,
                ),
              ),
              ..equations.zip(color-operations).flatten(),
            )
          })
        ]
      ]
    })
  }
}

"##;
