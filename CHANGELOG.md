# Changelog

## [0.12.7] - TBD

### Changed
- Renamed `standard_powers` to `integer_exponents` 

## [0.12.6] - 2026-08-08 

### Added
- f(x) graph problems
- `dot_with_lines` for graphs
- Graph legend support
- Support for quadratic functions
- You can now change `y_min` or `y_max` separately for `Axes`
- It's now possible to add custom objects to graphs
- Added `is_less()` and `is_greater()` for `InequalitySign`

## [0.12.5] - 2026-08-07

### Added
- Inequality sign support
- Inequality problems
- Text variant f(x) problems
- More granular support for major ticks and major gridlines in Axes

### Changed
- New `picker` algorithm: add one of each problem if possible, gaussian on the rest
- Removed words `add` and `with` in `SolutionWithSteps` methods for brevity
- New split marker: `[[split_one || split_two]]` and support for markers of any length

### Fixed
- Equation solutions are centered even when text is present
- Removed possible numerator integers in `divide_one_decimal_integer`

### Removed
- No more minor ticks for `large_k` problem

## [0.12.4] - 2026-08-05

### Added
- Added `12/30` and `12/300` problems
- Added more dynamic `ProblemParameters` struct for more ergonomic problem generation
- Support for subquestions and subanswers outside of prefix grouping
- Support for subdivisions in DB q/a/s strings
- More ergonomic `replace_one()` in addition to `replace_placeholders()` (now `replace_multiple()`)

### Changed
- Changed `decimal().with_decimals(N)` to `decimals(N)` in `num_gen`

### Fixed
- `.decimals()` no longer prints an error message when used on an integer
- `two_digit_to_decimal_form` outputs in math mode correctly
- Axes are ticked more appropriately
- `find_m` axes are scaled better when `m = 0`

## [0.12.3] - 2026-08-03

### Added
- Implemented `as_integer()` on `Numbers`
- Implemented `decimals()` on `Numbers`
- Added `Solution::block_with_text()` solution builder
- Made a common `NumberGenerator` trait
- Added decimal multiplication and division problems
- Large integers are now displayed with a thousand separating space: `12 345`
- Implemented `space()` and `wide_space()` on `ContinuousSolution`

### Fixed
- Fixed bug where specifying the number of decimals printed would not work if the number was an integer

## [0.12.2] - 2026-08-02

### Added
- Implemented `sqrt()` and `root(n)` on `Numbers`
- Implemented `extend(n)` on `Numbers`
- Implemented `can_be_simplified()` on `Numbers`
- Added non-integer exponent problems
- Added `Solution::inline()` and `Solution::block()` solution builders
- Added `as_block_math()` to `MathDisplay`

### Changed
- Updated `README`
- Renamed `Question` to `QuestionString` and so on
- Changed `SolutionWithSteps::new()` API to `Solution::with_steps()` to accomodate for other type of solution structs

### Fixed
- Creating a new entry in the editor works properly again
- When a new topic is created, the linked problems get their difficulties set properly

## [0.12.1] - 2026-07-30

### Added
- Added `order_of_operations` problems
- Added support for `fill`/`border`/`none` for solution background
- Added support for changing solution colors
- You can now search for `public` or `private` in the web editor
- The "publish every X" button only shows when there is something to publish
- Web editor list auto-refreshes when an entry is changed
- Backend prints whether it's in dev or prod mode during startup
- User preferences with regards to document options are now saved

### Changed
- Renamed `web_editor` to `editor` and `stats_page` to `dashboard`

### Fixed
- Copies in editor now load their related entries properly
- Fixed reordering bug in editor lists - as a bonus the editor no longer queries the backend on every reorder

## [0.12.0] - 2026-07-25

### Added
- The `public` field is editable in the editor, and private entries are highlighted in the list
- Made the entry list in the web editor adapt to narrower screens
- Added API and editor button for making every entry public

### Changed
- Extensive refactor of `db`:
  - Refactored `db` types by putting them with their respective functions for reduced cognitive load and easier expanding in the future
  - Made functions for the editor return the `public` field, while user functions don't
  - Program scans for `production_mode` just once and then stores it in a `Lazy`
  - `get_X_from_ids` have been replaced with `get_X_from_Y_id` to reduce data sending back and forth
  - Optimized every load of the entry list in the editor by just hitting the DB twice in total, instead of once (or twice!) for every entry

## [0.11.1] - 2026-07-23

### Added
- All parts of the course structure now has `public` flag in the DB
- When in production mode, we only fetch `public` data. This allows for dev work on the same database without affecting the website

### Changed
- Moved evaluating of expressions into its own module, and simplifying into its own.
- Changed `log` flag to `prod` since the changes above will make the flag behave like the prod build in more ways than just logging

### Fixed
- The statistic `most_changed_fields` mistakenly compared every set against each other in two PDFs, leading to misleading stats

## [0.11.0] - 2026-07-22

### Added
- Mobile layout support

### Fixed
- `SetEditor` components no longer stack on top of each other in layout view on narrower screens
- When sets are created, they now copy the set options properly instead of connecting to the state

## [0.10.0] - 2026-07-20

### Changed 
- Adjusted `picker` algorithm to no longer mix problems across difficulties
- `picker` works differently when only one topic is included in the set - it now splits more evenly across problems instead of doing a predetermined difficulty curve
- Text variants are now evaluated after problem generation, leading to predictable cycling of the variants, even when multiple threads run the same problem at the same time. Praise be!

### Fixed
- Reformatted comment that was mistaken for a doc-test

## [0.9.3] - 2026-07-19

### Added
- Additional problems about negative numbers
- Shortcut functions for the registry: `get_question()`, `get_answer()`, `get_solution()`
- Implemented trait `MathDisplay` for every `Display` type for easier `"${num}$` displays

### Changed
- Renamed `extract_identifiers()` to `convert_identifiers_to_i32s()` for clarity

### Fixed
- Reorder icon now changes color with the theme

## [0.9.2] - 2026-07-18

### Added
- Support for multiple variants in problem texts: `The price of [flour || milk || cheese] has gone up.`
- Support for stats about what is/isn't changed between reloads of a PDF
- New leaderboards in stats page showing the above-mentioned stats
- New `/defaults` endpoint where the frontend queries the backend about the default values for fields, to have a single source of truth
- Search bar in web editor can now look at more fields

### Fixed
- Polynomial terms where all variables have an exponent of 0 and an `|coefficient| == 1` are now rendered correctly
- Adressed some height issues in web editor

### Changed
- Moved types `Question`, `Answer` and `Solution` to `types` crate

### Removed
- Removed some unused functions in backend (crate `types`)
- Dangling vite svg file

## [0.9.1] - 2026-07-14

### Added
- Conditional logging based on cargo feature flags (Use `cargo run -- log` to opt-in when in dev mode)

### Fixed
- Renamed `set_options` in frontend to `formatting_options` to match backend
- Fixed double scrollbars on stats page by only using padding in `<main>`
- Fixed Typst crash when margins were changed
- Fixed incorrect parentheses when solving pq-style equations
- Color option now works properly for solutions. Graphs are still always colored.

### Changed 
- Increased padding for leaderboard counts and widened leaderboards on the stats page
- Shortened tracing logs for readability
- Colors in Typst are now written with hex code

## [0.9.0] - 2026-07-12

### Added
- A stats page where you can see various stats about PDFs and user preferences! This required a lot of work:

#### Logging
- Log entries for everything surrounding the PDFs are stored in the DB
- Also logs language and course API calls for statistics
- To track what the user changes between renders, the server returns the id of the logged PDF. The frontend then sends that ID in the request of an eventual new PDF

#### Stats
- A `/stats` API endpoint (protected by auth) which collates logs from the DB
- Leaderboard endpoints for topic inclusion and problem exclusion

### Changed 
- Moved common styling elements to the `App.css` file
- Moved all api functions to the `server` crate
- Refactored the `router` module to use path nesting for legibility

## [0.8.1] - 2026-07-07

### Added
- HTTP endpoint /lang/course is now cached for 10 mins to minimize refetching during active session
- Subtitles (as in a subheading) are now supported
- A field for writing your name can now be added by the user

### Changed
- Images for README now lives on GitHub CDN and not in repo
- The `heading` field for set options and `title` field for document options can now be None(rust)/null(js). Default changed to null instead of an empty string.
- The backend now fetches every entry in the i18n_pdf table, instead of fetching specific keys. This prevents typing out new entries multiple times.
- When a translation isn't found for the PDF, an error message is printed in the PDF instead of the program crashing with expect().

### Removed
- Headings are no longer sent to the custom Typst column function for measurement, since the "auto page break" feature is long gone.

### Fixed
- Custom headings before problem sets now properly overwrite prefixes
- Custom titles are now properly sanitized

## [0.8.0] - 2026-07-05
### Added
- Layout view where a rough layout of the stencil can be viewed and edited

### Changed
- Removed caching of translations in localStorage, let HTTP caching handle it
- Padding increased on Move Up/Move Down buttons
- Fields ´problems´ and ´options´ in the HTTP API have been changed to ´problem_options´ and ´set_options´
- The descriptions for topics and problems now live in the global state for easier access for multiple components

### Fixed
- Prevented Typst injections when adding custom text to stencil
- Label "File name" in PDF View now changes with the language
- SetCard icons (delete, etc.) no longer appear when navbar is closed

## [0.7.3] - 2026-06-30
### Added
- Delete icon on SetCard
- Reorder icon on SetCard with both click and drag functionality
- Error log when no Authorization header is present at all

### Changed
- Swish QR image optimized from 125 kB to 8 kB.

### Fixed
- Removed dangling CSS style
- Removed svelte router plugin
- Updated web_editor package-lock.json
- Dev path for server in web_editor updated (Did it break production? We'll see 🥰) [It didn't!]

## [0.7.2] - 2026-06-27
### Added
- New "Contact me" popover with email link and Swish QR

### Changed
- Settings menu is now a native HTML popover element

## [0.7.1] - 2026-06-26
### Changed
- Updated README.md, including example images

## [0.7.0] - 2026-06-26
The first official GitHub release of `stencil`! This is version 0.7.0 since many many internal versions have been had before this proper Github release.
I'm also publishing it now, in its unfinished state, to test the GitHub Actions CI.

### Added
- Started changelog
- Dynamic `<title>` in html
- Rebrand from `Stencil` to `stencil.nu` in frontend
- Added styling to EditSetView and made sure it and AddSetView is consistent with each other designwise
- Added footer with options and a delete button to EditSetView

### Fixed
- Frontend connects to backend in dev mode again
- Favicon is now persistent after build
- AddSetView now starts at the top when content is too tall


