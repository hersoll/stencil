#let colored(x) = text(fill: color.linear-rgb(22%, 10%, 33%), $#x$)
#let primary(x) = text(fill: color.linear-rgb(9%, 3%, 18%), $#x$)
#let secondary(x) = text(fill: color.linear-rgb(22%, 10%, 33%), $#x$)
#let tertiary(x) = text(fill: color.linear-rgb(30%, 23%, 39%), $#x$)

#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)
#show math.equation.where(block: false): box
#let item-counter = counter("item-counter")
#let bold-alpha = n => [*#numbering("a)", n)*]

#let balanced(column_count, items, start_pos, custom_spacing: 1.33em, title: [], debug: false) = layout(
  size => {
    let spacing = custom_spacing.to-absolute()
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
    block(height: current_height)[
      #set enum(spacing: spacing)
      #columns(column_count, gutter: gutter, enum(..items))
    ]

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
        inset: 0.5em,
        align: (left, horizon + left),
        grid.vline(x: 1, stroke: (paint: linecolor, thickness: 0.5pt)),
        ..equations.zip(color-operations).flatten(),
      )
    })
  }
}
