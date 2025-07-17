pub static PREAMBLE_STR: &str = r#"
#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)

#show math.equation.where(block: false): box

#let will_fit(column_height, column_count, heights, spacing) = {
    let accumulated_height = 0pt
    let pretend_columns = 1
    for height in heights {
      accumulated_height += height + spacing

      if accumulated_height > column_height and pretend_columns < column_count {
        accumulated_height = height
        pretend_columns += 1
      }
    }
   accumulated_height <= column_height 
}
#let balanced(column_count, items, spacing) = layout(size => {
  let gutter = 1em
  let column_width = (size.width - gutter * (column_count - 1)) / column_count

 let heights = items.enumerate().map(((index, item)) => {
    let single_enum = enum(start: index + 1, item)
    measure(block(width: column_width, single_enum)).height
  }) 

  let total_height = heights.sum() + (items.len() - 1) * spacing
  
  let min_height = total_height / column_count
  let max_height = total_height
  // Binary search loop
  while max_height - min_height > 1pt {
    let mid_height = (min_height + max_height) / 2
    if will_fit(mid_height, column_count, heights, spacing) {
      max_height = mid_height
    } else {
      min_height = mid_height
    }
  } 

  block(height: min_height)[
    #columns(column_count, gutter: gutter, enum(..items))
  ]
  // [
  //   Paper height: #size.height \
  //   Enums: #heights.sum() \
  //   Spacing: #((items.len() - 1) * spacing)\
  //   Total height: #total_height \
  //   Target height: #column_height \
  //   Final height: #final_column_height
  // ]
})

//Colors
#let colored(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)
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
