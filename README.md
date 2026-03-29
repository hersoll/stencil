# Stencil
Generate math problems programmatically and turn them into a problem sheet (swe: _stencil_).

Built with Rust, Svelte and Postgres.

## Backend
The backend is responsible for generating and distributing problems across the stencil, as well as writing the Typst file and compiling it.

The backend consists of five main crates:
- problem_generator
- math
- db
- typst_writer
- server

### server
Built on `axum`, the public-facing API has two main endpoint categories: the PDF endpoints and the Course endpoints.
The PDF endpoints are for generating PDFs (no way!) and the Course endpoints are for retrieving data about courses, chapters, topics and problems.
The two categories are rate-limited with different limits.

#### PDF API
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

#### Course API
The Course API can be accessed via the `/api/{lang}/{course}/{chapter}/{topic}` endpoint:

- `/api/sv/course/ma1b` - get (in Swedish) every chapter in ma1b, every topic in each of those chapters (NOT every problem, to lessen network and memory usage)
- `/api/en/course/ma2b/quadratics` - get (in English) every topic from the quadratics chapter
- `/api/sv/course/ma1c/1` - get every topic from the chapter with an ID of 1 (alternative to doing it by name)
- `/api/sv/course/ma1b/functions/f_x` - get every problem in the f(x) topic

## Frontend
Svelte
