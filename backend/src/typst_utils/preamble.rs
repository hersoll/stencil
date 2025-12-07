pub static PREAMBLE_STR: &str = r#"
#import "/preamble.typ": *
#import "@preview/equate:0.3.2": equate, share-align
#import "@preview/zero:0.5.0": num, set-num
#show: equate.with(debug: false)
#show math.equation.where(block: false): box
#set enum(spacing: 1.33em)
#set enum(numbering: it => {
  item-counter.step()
  context box(width: 1.35em, text(weight: "bold")[#item-counter.display()])
})
#set-num(decimal-separator: ",")
"#;
