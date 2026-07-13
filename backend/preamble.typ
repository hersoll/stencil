// Used for equation separator
#let line_color = color.linear-rgb(20%, 20%, 20%)
#let colored(x) = text(fill: color.linear-rgb(22%, 10%, 33%), $#x$)

#import "@preview/equate:0.3.2": equate, share-align
#show: equate.with(debug: false)
#show math.equation.where(block: false): box
#let item-counter = counter("item-counter")
#let bold-alpha = n => [*#numbering("a)", n)*]
#let margin-top(margin) = { if type(margin) == dictionary { margin.top.length } else { margin } }

#let balanced(column_count, items, start_pos, custom_spacing: 1.33em, debug: false) = layout(
  size => {
    let spacing = custom_spacing.to-absolute()
    // How many pts we start at
    let start_height = start_pos - margin-top(page.margin)
    // How many pts we have available to use on this page
    let spare_height = size.height - start_height

    let gutter = 1em
    let column_width = (size.width - gutter * (column_count - 1)) / column_count

    let heights = items.enumerate().map(
      ((index, item)) => {
        // Make sure we account for two-digit items, otherwise it might allow "too much" horizontal space while measuring
        let single_enum = enum(start: index + 11, item)
        measure(block(width: column_width, single_enum)).height
      },
    )

    // The height if we would stack all items in one column with proper formatting
    let total_height = heights.sum() + (items.len() - 1) * spacing
    // Best case: perfect split across columns
    let min_height = total_height / column_count
    // Worst case: Everything in one column
    let max_height = total_height

    // current_height is the height we want our columns to be in the end
    let current_height = min_height

    // This loop slowly increases the column height until it finds a height that works.
    while current_height < max_height {
      let available_height = spare_height
      let current_column_height = current_height

      // How much we can fit in this current iteration
      let limit = calc.min(current_height, available_height)

      // The height of each column on the **current** page
      // The final height will be chosen from the max of these heights
      let column_heights = ()

      // Let's check how much we can fit in the current space!
      // Starting with some setup:
      let current_column = 1
      let accumulated_height_in_current_column = 0pt
      // Variable used to keep track of whether we have overflowing or not
      let can_fit = true
      // ?
      let empty_space = 0pt

      let item_index = 0
      while item_index < heights.len() {
        let current_item_height = heights.at(item_index)

        // If our item is too tall for the space to begin with, we obviously need more space. Go to the next iteration.
        if current_item_height > limit {
          can_fit = false
          break
        }
        // Otherwise, we add it to the current_column as a test and do some checks below
        accumulated_height_in_current_column += current_item_height

        // Check if we need to move to next column
        if accumulated_height_in_current_column > limit and current_column < column_count {
          // Remove the latest item from the column and submit the column for peer review
          column_heights.push(accumulated_height_in_current_column - current_item_height - spacing)
          current_column += 1
          accumulated_height_in_current_column = -spacing // -spacing since the end of the iteration adds spacing
          item_index -= 1 // Recheck the overflowing item
        }

        // We've reached the end of the page, handle page breaks
        //
        // This condition won't be met while there are columns to fill,
        // since the previous condition will always reset accumulated_height
        //
        // NOTE: Could be optimized? the current_column_height > available_height check could maybe be changed?
        if accumulated_height_in_current_column > limit and current_column_height > available_height {
          // Reset the distribution for the remaining items
          column_heights = ()
          accumulated_height_in_current_column = -spacing // -spacing since the end of the iteration adds spacing
          item_index -= 1 // Re-check the overflowing item
          current_column_height -= limit // We've used up limit worth of space
          limit = calc.min(current_column_height, size.height)
          available_height = size.height
          current_column = 1 // Reset to first column on new page
        }

        accumulated_height_in_current_column += spacing
        item_index += 1
      }
      accumulated_height_in_current_column -= spacing

      if can_fit {
        // The final column is never added, do it here
        column_heights.push(accumulated_height_in_current_column)

        // This is only an approved height if the last column doesn't overshoot too much
        // (which it will do if we have columns that are too short)
        if column_heights.len() <= column_count and column_heights.first() >= column_heights.last() - 4em.to-absolute() {
          break //approved, ship it!
        }
      }

      current_height += 4pt
    }

    block(height: current_height)[
      #set enum(spacing: spacing)
      #columns(column_count, gutter: gutter, enum(..items))
    ]

    if debug {
      [
        Paper height: #size.height \
        Total height if one column: #total_height \
        Available height: #spare_height \
        Final height of box: #current_height \
        Minimum height: #start_height \
        Reported start_pos: #start_pos \
        Spacing: #spacing.pt() pt \
        Item heights: #heights.map(h => str(calc.round(h / 1pt, digits: 1))).join(", ") pt \
      ]
    }
  },
)

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
        grid.vline(x: 1, stroke: (paint: line_color, thickness: 0.5pt)),
        ..equations.zip(color-operations).flatten(),
      )
    })
  }
}
