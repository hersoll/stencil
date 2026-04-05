# Plan
Track usage with a postgres db
Only track in production - use #[cfg]!

# What to track:
- PDF document options (paper size, problem language, solutions shown, etc.)
- Problem selection in PDFs
- Set count, number of problems, difficulty
- Time of API call
- Frontend calls to /translations - which language? (Can track counts instead of individual calls)
- Theme

## Desired API
```rust 
// The struct used in API calls to PDF generation
pub struct ProblemOptions {
    /// Topics to draw problems from
    pub topics: Vec<i32>,
    /// Which problems to exclude from the generations
    #[serde(default)]
    pub exclusions: Vec<i32>,
    pub starting_difficulty: Difficulty,
    pub ending_difficulty: Difficulty,
    /// Number of problems
    pub n: u8,
}
// We also get a Vec<Problem> at the end of generator::generate_problem_set

metrics::pdf(problems, options);
metrics::translation(lang);
metrics::theme(theme);
```
