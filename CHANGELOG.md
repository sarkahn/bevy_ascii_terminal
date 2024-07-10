# Changelog

## [0.16] - 2024/07/09

### Changed
- `put_string` now aligns all strings to the top-left by default. You can manually specify a `Pivot` to override this.
- `put_string` in addition to wrapping on newlines, will now word wrap by default. You can override this by using the `StringFormatter` trait.  
- All `FormattedTile` functionality moved to `Terminal::format_tile`.
- `ToWorld` has been replaced by `TerminalTransform`, which can be used to transform positions between world space and terminal grid points. See the "transform" example. 

### Removed
- Dependencies on the grid and tiled camera crates has been removed.
- "Auto camera" functionality is now covered by `TerminalCamera`/`TerminalCameraBundle`.
- Geometric grid types have been moved into the `Grid` module.
- The border is no longer separate from the terminal.

### Added
- Most terminal functions can be called directly from `TerminalBundle` to simplify initial terminal setup.