# Stencil - A tool for creating math problem sheets

## Examples
Each set (section of the PDF page) is built by the `SetBuilder`, which uses a builder pattern. The `SetBuilder` 
takes its problems from defined problem areas (e.g. `SimpleEquation` or `GeneralQuadraticEquation`)
given to it in the code and constructs batches with specified difficulties.

### Shortest example
Each set requires at least one `area` and one `batch`. Each batch has a difficulty (`Intro`, `Easy`, `Medium` or `Hard`) and a number of problems.
The set is returned in the form of a `Vec<Problem>`.
```rust
//Will generate a Vec with one Problem
let equations = stencil::SetBuilder::new()
        .area(stencil::problems::SimpleEquations)
        .batch(Difficulty::Intro, 1)
        .build();
let equation = equations.iter().first().unwrap();
println!("{}", equation.question());
println!("{}", equation.answer());
```

### Multiple batches
Multiple batches can be chained and still be part of the same set (the same section in the actual PDF). The order of the batches is preserved.
```rust
let equations = stencil::SetBuilder::new()
        .area(stencil::problems::SimpleEquations)
        .add(Difficulty::Intro, 2)
        .add(Difficulty::Easy, 3)
        .add(Difficulty::Medium, 2)
        .add(Difficulty::Hard, 1)
        .build();
while let Some(equation) = equations.iter().next() {
    println!("{}", equation.question());
    println!("{}", equation.answer());
}
```

### Excluding types of questions
If you want to exempt a type of problem, you can call `exclude()` on the builder. 
The problem types are accessed through the `area` struct.
Exclusion can happen at any time before `build()`, even before the related area is added!

```rust
use stencil::problems::*
let equations = stencil::SetBuilder::new()
    .area(SimpleEquations)
    .exclude(SimpleEquations::ONLY_MULTIPLICATION)
    .add(Difficulty::Intro, 5)
    .build();
```
