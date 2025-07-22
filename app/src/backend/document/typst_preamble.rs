pub static PREAMBLE_STR: &str = r#"
#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)

#show math.equation.where(block: false): box

//Enum settings
#let item-counter = counter("item-counter")
#set enum(numbering: it => {
  item-counter.step()
  context box(width: 1.35em, text(weight: "bold")[#item-counter.display())])
})
#set enum(spacing: 6mm)

#let balanced(column_count, items, spacing, start_pos, title: [], debug: false) = layout(
  size => {
    let start_height = start_pos - page.margin.length
    let spare_height = size.height - start_height

    let gutter = 1em
    let column_width = (size.width - gutter * (column_count - 1)) / column_count

    let title_height = if title == "" { 0pt } else { measure(block(width: size.width, title)).height }
    let heights = items.enumerate().map(((index, item)) => {
      let single_enum = enum(start: index + 11, item)
      measure(block(width: column_width, single_enum)).height
    })

    let total_height = heights.sum() + (items.len() - 1) * spacing
    let min_height = total_height / column_count
    let max_height = total_height

    // Push to next page if it's relatively small and we can't fit it on the page
    // OR if we can't fit at least three rows of a long set
    let pre_spacing = if (min_height > spare_height and min_height < size.height / 4) or (title_height + calc.max(..heights)) * 3 > spare_height {
      spare_height
    } else {
      0pt
    }
    // Adjust spacing if we push to next page
    let effective_spare_height = if pre_spacing > 0pt {
      size.height
    } else {
      spare_height
    }

    let current_height = min_height

    while current_height < max_height {
      let available_height = effective_spare_height
      let limit = calc.min(current_height, available_height)
      let current_column_height = current_height
      let column_heights = ()
      let current_column = 1
      let accumulated_height = 0pt
      let can_fit = true
      let empty_space = 0pt

      for height in heights {
        if height > limit {
          can_fit = false
          break
        }

        accumulated_height += height

        // Check if we need to move to next column
        if accumulated_height > limit and current_column < column_count {
          column_heights.push(accumulated_height - height - spacing)
          current_column += 1
          accumulated_height = height
        }
        // Handle page breaks
        if accumulated_height > limit and current_column_height > available_height {
          column_heights.push(accumulated_height - height)
          let section_height = calc.max(..column_heights)
          empty_space += limit - section_height

          column_heights = ()
          accumulated_height = height
          current_column_height -= (limit - empty_space - spacing)
          limit = calc.min(current_column_height, size.height)
          available_height = size.height
          current_column = 1 // Reset to first column on new page
        }

        accumulated_height += spacing
      }
      accumulated_height -= spacing

      if can_fit {
        column_heights.push(accumulated_height)

        if column_heights.len() == column_count and column_heights.first() >= column_heights.last() {
          break
        }
      }

      current_height += 4pt
    }

    v(pre_spacing)
    if title != [] {
      title
    }
    block(height: current_height)[ // Use current_height, not max_height
      #set enum(spacing: spacing)
      #columns(column_count, gutter: gutter, enum(..items)) ]

    if debug {
      [
        Paper height: #size.height \
        Total height: #total_height \
        Min height: #min_height \
        Max height: #max_height \
        Current height: #current_height \
        Starting height: #start_height \
        Available height: #spare_height \
      ]
    }
  },
)

//Colors
#let colored(x) = text(fill: color.linear-rgb(10%, 10%, 10%), $#x$)
#let linecolor = color.linear-rgb(20%, 20%, 20%)

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

// #let balanced(column_count, items, spacing, start_pos, title: [], debug: false) = layout(
//   size => {
//     let start_height = start_pos - page.margin.length
//     let spare_height = size.height - start_height
//
//     let gutter = 1em
//     let column_width = (size.width - gutter * (column_count - 1)) / column_count
//
//     let title_height = if title == "" { 0pt } else { measure(block(width: size.width, title)).height }
//     let heights = items.enumerate().map(((index, item)) => {
//       let single_enum = enum(start: index + 11, item)
//       measure(block(width: column_width, single_enum)).height
//     })
//     let total_height = heights.sum() + (items.len() - 1) * spacing
//     let min_height = total_height / column_count + title_height
//     let max_height = total_height + title_height
//
//     // Push to next page if it's relatively small and we can't fit it on the page
//     // OR if we can't fit at least three rows of a long set
//     let pre_spacing = if (min_height > spare_height and min_height < size.height / 4) or calc.max(..heights) * 3 > spare_height {
//       spare_height
//     } else {
//       0pt
//     }
//     // Adjust available height if we push to next page
//     let effective_spare_height = if pre_spacing > 0pt {
//       size.height // If we're pushing to next page, use full page height
//     } else {
//       spare_height // Otherwise use remaining height on current page
//     }
//     // Binary search loop
//     while (max_height - min_height).abs > 1pt {
//       let available_height = effective_spare_height
//       let mid_height = (min_height + max_height) / 2
//       let limit = calc.min(mid_height, available_height)
//       let current_column_height = mid_height
//       let accumulated_height = title_height
//       let current_column = 1
//       let can_fit = true
//       for height in heights {
//         if height > limit and current_column_height == mid_height {} else if height > limit {
//           can_fit = false
//           min_height = mid_height
//           break
//         }
//         accumulated_height += height
//
//         if accumulated_height > limit and current_column < column_count {
//           accumulated_height = height
//           current_column += 1
//         }
//         if accumulated_height > limit and current_column_height > available_height {
//           accumulated_height = height
//           current_column_height -= limit
//           limit = calc.min(current_column_height, size.height)
//           available_height = size.height
//           current_column = 1
//         }
//         accumulated_height += spacing
//       }
//       accumulated_height -= spacing
//
//       if accumulated_height <= current_column_height and can_fit {
//         max_height = mid_height
//       } else {
//         min_height = mid_height
//       }
//     }
//     v(pre_spacing)
//     if title != [] {
//       title
//     }
//     block(height: max_height)[
//       #columns(column_count, gutter: gutter, enum(..items))
//     ]
//     if debug {
//       [
//         Paper height: #size.height \
//         Total height: #total_height \
//         Min height: #min_height \
//         Max height: #max_height \
//         Starting height: #start_height \
//         Avaliable height: #spare_height \
//       ]
//     }
//   },
// )
