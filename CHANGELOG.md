# Changelog

## [Unreleased]
### Changed
- Swish QR image optimized from 125 kB to 8 kB.

### Fixed
- Removed dangling CSS style
- Removed svelte router plugin

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

### Roadmap
To remind future me, here's the summer roadmap: 
- **0.7.2:** "Contact me" popover 
- **0.8.0:** Layout page
- **0.9.0:** Mobile layout
- **0.10.0:** Metrics collection and stats webpage
- **1.0.0:** Release! 😵‍💫 Add as many problems as humanly possible.
