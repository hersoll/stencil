# Stencil - A tool for creating math problem sheets

## Examples
The library consists of a large number of builders that all construct the same object, a `Vec<Problem>`. 
Hence, the builders are not called `...Builder` but are simply referred to by a descriptor of the type of problem they generate (e.g. `SimpleEquation` or `GeneralQuadraticEquation`).

### Shortest example
Stencil constructs problem sets in batches. Each batch has a difficulty (`Intro`, `Easy`, `Medium` or `Hard`) and a number of problems.
```rust
//Will generate a Vec with one Option<Problem>
let equations = stencil::problems::SimpleEquation::new()
        .add(Difficulty::Intro, 1)
        .build();
let equation = equations.first().unwrap();
println!("{}", equation.question());
println!("{}", equation.answer());
```

### Multiple batches
Multiple batches can be chained and still be part of the same set (the same section in the actual PDF). The order of the batches is preserved.
```rust
let equations = stencil::problems::SimpleEquation::new()
        .add(Difficulty::Intro, 2)
        .add(Difficulty::Easy, 3)
        .add(Difficulty::Medium, 2)
        .add(Difficulty::Hard, 1)
        .build();
while let Some(equation) = equations.next() {
    println!("{}", equation.question());
    println!("{}", equation.answer());
}
```

### Excluding types of questions
If you want to exempt a type of problem, you can call `exclude()` on the builder. 
The problem types are accessed through an enum with the plural name of the problem category (`GeneralQuadraticEquation` -> `GeneralQuadraticEquations`).
Exclusion can happen at any time before `build()`

```rust
use stencil::problems::*
let equations = SimpleEquation::new()
    .exclude(SimpleEquations::OnlyAdditionAndSubtraction)
    .add(Difficulty::Intro, 5)
    .build();
```
