# TODO

- Metrics for changing PDFs
- Tooltips
- Adjust selection algorithm
- Mobile layout

# Changelog

## [0.9.2] - TBD

### Fixed
- Polynomial terms where all variables have an exponent of 0 and an `|coefficient| == 1` are now rendered correctly

### Removed
- Removed some unused functions in backend (crate `types`)

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


