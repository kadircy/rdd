# Changelog
## [Unreleased]

## [0.1.1] - 2025-02-26
### Added
- Better error handling in `check` method with better error messages.
- Handle error if the `dd --version` format isn't as expected.
- New helper method `set_args` for more modular and greater code.

### Changed
- Use `Vec<String>` in options instead of `Vec<(String, String)>` for better performance and greater code.
- Combine `match output` and `match output.stdout` into single block.

## [0.1.0] - 2025-02-04

### Added
- Everything
