# Changelog

<!-- https://keepachangelog.com/en/1.0.0/ -->

## [0.2.2] - 2023-04-06

### Changed

- Bump dependencies

## [0.2.1] - 2023-03-21

### Fixed

- Actually kill timed-out subprocesses

## [0.2.0] - 2023-03-16

### Added

- Flags for stdout/stderr regexes
- Support for JavaScript
- `--timeout`
- Test with `lit`

### Changed

- Improved error message for initially-uninteresting inputs
- Improvements to library API, move multi-pass reduction into the library
- Updated benchmarks

### Fixed

- Map Unix signals to exit codes like Bash does

## [0.1.0] - 2023-03-11

Initial release!

[0.1.0]: https://github.com/langston-barrett/treereduce/releases/tag/v0.1.0
[0.2.0]: https://github.com/langston-barrett/treereduce/releases/tag/v0.2.0
[0.2.1]: https://github.com/langston-barrett/treereduce/releases/tag/v0.2.1
[0.2.2]: https://github.com/langston-barrett/treereduce/releases/tag/v0.2.2