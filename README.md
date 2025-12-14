# Stencil

Generate math problems programmatically and turn them into a problem sheet (swe: _stencil_).

## Backend
Built using Rust (`axum`), the backend is responsible for generating and distributing problems across the stencil, as well as writing the Typst file and compiling it.

When a HTTP request to generate a stencil is recieved, the HTTP request contains two things: a list of sets and some document-spanning options (`DocumentOptions`), which includes things like language and title.
Each set has the following spec (`ProblemSetSpec`):
- Which topics the problems should come from
- Any problems to exclude from the set
- The starting difficulty
- The ending difficulty
- How many problems the set contains
- Special information about how to render this particular set (`SetOptions`), like:
    - Number of columns
    - Subtitle
    - Spacing between problems
    - etc.

## Frontend
React
