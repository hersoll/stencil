pub static PREAMBLE_STR: &str = r#"
#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)

#show math.equation.where(block: false): box

#let balanced(column_count, items, spacing, lines, debug: false) = layout(size => {
  let start_height = here().position().y - page.margin.length
  let spare_height = size.height - start_height
  let pre_spacing = 0pt

  let gutter = 1em
  let column_width = (size.width - gutter * (column_count - 1)) / column_count

  let heights = items.enumerate().map(((index, item)) => {
    let single_enum = enum(start: index + 11, item)
    measure(block(width: column_width, single_enum)).height
  })

  let total_height = heights.sum() + (items.len() - 1) * spacing
  let min_height = total_height / column_count
  let max_height = total_height
  // Binary search loop
  while (max_height - min_height).abs > 1pt {
    let available_height = spare_height
    let mid_height = (min_height + max_height) / 2
    let limit = if mid_height > available_height { available_height } else { mid_height }
    let current_column_height = mid_height
    let accumulated_height = 0pt
    let current_column = 1
    let can_fit = true
    for height in heights {
      if height > limit and current_column_height == mid_height {
        pre_spacing = limit
      } else if height > limit {
        can_fit = false
        min_height = mid_height
        break
      }
      accumulated_height += height

      if accumulated_height > limit and current_column < column_count {
        accumulated_height = height
        current_column += 1
      }
      if accumulated_height > limit and current_column_height > available_height {
        accumulated_height = height
        current_column_height -= limit
        limit = calc.min(current_column_height, size.height)
        available_height = size.height
        current_column = 1
      }
      accumulated_height += spacing
    }
    accumulated_height -= spacing

    if accumulated_height <= current_column_height and can_fit {
      max_height = mid_height
    } else {
      min_height = mid_height
    }
  }
  v(pre_spacing)
  block(height: max_height)[
    #if lines {
      for i in range(column_count - 1) {
        let line_x = (i + 1) * (size.width / column_count) - 0.25pt + gutter/4 * i
        place(dx: line_x, dy: 0pt, line(length: max_height, stroke: 0.5pt + gray, angle: 90deg))
      }
    }
    #columns(column_count, gutter: gutter, enum(..items))
  ] 
  if debug {
    [
      Paper height: #size.height \
      Total height: #total_height \
      Min height: #min_height \
      Max height: #max_height \
      Starting height: #start_height \
      Avaliable height: #spare_height \
    ]
  }
})

//Colors
#let colored(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)
#let linecolor = color.linear-rgb(20%, 20%, 20%)

//Enum settings
#let item-counter = counter("item-counter")
#set enum(numbering: it => box(width: 1.35em, text(weight: "bold")[#it)]))
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

    let color-operations = operations.map(op => if op != $$ { colored(op) } else { op })

    share-align({
      grid(
        columns: (max-eq-width, auto),
        inset: 5pt,
        align: (left, horizon + left),
        grid.vline(x: 1, stroke: (paint: linecolor, thickness: 0.5pt)),
        ..equations.zip(color-operations).flatten(),
      )
    })
  }
}
"#;
