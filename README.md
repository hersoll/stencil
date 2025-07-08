# Stencil (Work in progress)

A Rust library for generating opinionated math problem sets and rendering them to PDF using Typst.

## Features

- **Flexible Problem Generation:** Generate math problems with configurable difficulty levels
- **Hand-written solutions:** Every single problem has a custom step-by-step solution if wanted
- **Builder Pattern API:**: Intuitive interface for constructing problem sets
- **Multiple Math Areas (Under construction):** Sorted after the Swedish curriculum
- **PDF Output:** Renders (soon to be) beautiful problem sets using Typst
- **Extensible:** Easy to add new problem types

## Examples
Each set requires at least one `area` and one `batch`. Each `batch` has a difficulty (`Intro`, `Easy`, `Medium` or `Hard`) and a number of problems.
The set is returned in the form of a `Vec<Problem>`.

### Basic Usage

```rust
use stencil::{SetBuilder, Difficulty};
use stencil::problems::SimpleEquations;

// Generate a simple problem set
let problems = SetBuilder::new()
    .area(SimpleEquations)
    .batch(Difficulty::Easy, 5)
    .build();

// Print questions and answers
for problem in problems {
    println!("Q: {}", problem.question());
    println!("A: {}", problem.answer());
    println!("Solution: {}", problem.solution());
}
```

### Advanced Usage

```rust
use stencil::{SetBuilder, Difficulty};
use stencil::problems::*;

// Create a mixed difficulty set and write out the solution for each problem
let problems = SetBuilder::new()
    .area(SimpleEquations)
    .batch(Difficulty::Intro, 2)
    .batch(Difficulty::Easy, 3)
    .batch(Difficulty::Medium, 2)
    .batch(Difficulty::Hard, 1)
    .build();

// Exclude specific problem types
let problems = SetBuilder::new()
    .area(SimpleEquations)
    .exclude(SimpleEquations::ONLY_MULTIPLICATION)
    .batch(Difficulty::Easy, 5)
    .build();
```

### Generating PDFs

```rust
use stencil::{SetBuilder, DocumentBuilder, WriteSolutions};

// Create problems
let problems = SetBuilder::new()
    .area(SimpleEquations)
    .batch(Difficulty::Easy, 10)
    .build();

// Render to PDF
let document = DocumentBuilder::new()
    .heading("Math Practice Set")
    .write_solutions(WriteSolutions::All)
    .add_problem_set(problems)
    .file_name("practice_set")
    .build()?
    .compile()?;

```

## License

This project is licensed under the YourName Non-Commercial License.  
Commercial use is not allowed without permission. See `LICENSE` for details.
