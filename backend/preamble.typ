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
    // How many pts we start at
    let start_height = start_pos - page.margin.length
    // How many pts we have available to use
    let spare_height = size.height - start_height

    let gutter = 1em
    let column_width = (size.width - gutter * (column_count - 1)) / column_count

    let title_height = if title == "" { 0pt } else { measure(block(width: size.width, title)).height }
    let heights = items.enumerate().map(
      ((index, item)) => {
        // Make sure we account for two-digit items, otherwise it might allow "too much" horizontal space while measuring
        let single_enum = enum(start: index + 11, item)
        measure(block(width: column_width, single_enum)).height
      },
    )

    let total_height = heights.sum() + (items.len() - 1) * spacing
    let min_height = total_height / column_count
    let max_height = total_height

    // Push the entire set to next page if it's relatively small and we can't fit it on the page
    // OR if we can't fit at least three rows of a long set
    let at_page_top = start_height <= 1pt
    let pre_spacing = if not at_page_top and (
      (min_height > spare_height and min_height < size.height / 4) or (title_height + calc.max(..heights)) * 3 > spare_height
    ) {
      spare_height
    } else {
      0pt
    }

    // If we push to next page, we have the entire page avaliable to us
    let effective_spare_height = if pre_spacing > 0pt {
      size.height
    } else {
      spare_height
    }

    // current_height is the height we want our columns to be in the end
    let current_height = min_height

    // This loop slowly increases the column height until it finds a height that works.
    while current_height < max_height {
      let available_height = effective_spare_height
      let current_column_height = current_height

      // How much we can fit in this current iteration
      let limit = calc.min(current_height, available_height)

      // The height of each column on the **current** page
      // The final height will be chosen from the max of these heights
      let column_heights = ()

      let current_column = 1
      let accumulated_height = 0pt
      let can_fit = true
      let empty_space = 0pt

      let index = 0
      while index < heights.len() {
        let height = heights.at(index)

        if height > limit {
          can_fit = false
          break
        }

        accumulated_height += height

        // Check if we need to move to next column
        if accumulated_height > limit and current_column < column_count {
          column_heights.push(accumulated_height - height - spacing)
          current_column += 1
          accumulated_height = 0pt
          index -= 1 // Recheck the overflowing item
        }

        // We've reached the end of the page, handle page breaks
        //
        // This condition won't be met while there are columns to fill, since the previous condition will always reset accumulated_height
        if accumulated_height > limit and current_column_height > available_height {
          // Disregard the overflowing item
          column_heights.push(accumulated_height - height)

          // Reset the distribution for the remaining items
          column_heights = ()
          accumulated_height = 0pt
          index -= 1 // Re-check the overflowing item
          current_column_height -= limit // We've used up limit worth of space
          limit = calc.min(current_column_height, size.height)
          available_height = size.height
          current_column = 1 // Reset to first column on new page
        }

        accumulated_height += spacing
        index += 1
      }
      accumulated_height -= spacing

      if can_fit {
        // The final column is never added, do it here
        column_heights.push(accumulated_height)

        // This is only an approved height if the last column doesn't overshoot too much
        // (which it will do if we have columns that are too short)
        if column_heights.len() <= column_count and column_heights.first() >= column_heights.last() * 0.95 {
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
        Column height: #current_height \
        Starting height: #start_height \
        Available height: #spare_height \
        Spacing: #spacing.pt() pt \
        Item heights: #heights.map(h => str(calc.round(h / 1pt, digits: 1))).join(", ") pt \
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
        inset: (top: 0.2em, rest: 0.5em),
        align: (left, horizon + left),
        grid.vline(x: 1, stroke: (paint: linecolor, thickness: 0.5pt)),
        ..equations.zip(color-operations).flatten(),
      )
    })
  }
}
