# Changelog
## [0.18.0] - 2025/11/15

### Changes
- Update for bevy 0.17.
- Moved strings to their own module.
- Added a bounds check to put_string.

## [0.17.0] - 2025/03/09

### Changes
- Update for bevy 0.16

## [0.16.6] - 2025/03/09

### Changes
- Fixed a crash where string iterator would try to split a string between char boundaries.
- Minor tweaks to examples.

## [0.16.5] - 2025/03/07

### Changes
- Merged a pr to directly us a `Handle<Image>` in font switching, will probably replace `TerminalFont::Custom` with that at some point.
- Fixed a bug where `Terminal::put_string` would always panic if negative values were used for the `GridPosition`, even if those values would be in bounds, ie: from a centered pivot.
- Added `Terminal::read_line`.
- Cleanup docs.
- Add credits to readme.
  
## [0.16.4] - 2025/03/03

### Changes
- Moved `TerminalCamera`update systems to `First` schedule to fix the camera flicker any time the terminal was resized.

## [0.16.3] - 2025/03/03

### Changes
- Moved mesh update systems from the `Last` schedule to the `PostUpdate` schedule to fix a one-frame-delay bug.

## [0.16.2] - 2025/02/27

### Changes
- Removed usage of bevy's `embedded_asset!` macro as it causes crashes in windows wasm builds: https://github.com/bevyengine/bevy/issues/14246. Reverted to manually setting up image handles for built in fonts, no api change.
- Added necessary bevy dependencies so linux/wasm builds should work.
- Cargo update.

## [0.16.1] - 2025/02/25

### Changes
- Fixed a bug where the `TerminalCamera` update system would panic if the terminal font wasn't loaded yet.

## [0.16] - 2025/02/17

### Changes
- `put_string` now aligns all strings to the top-left by default. You can manually specify a `Pivot` to override this. In addition to wrapping on newlines, it will now word wrap by default. You can override this with the `dont_word_wrap` function.
- Dependency on `TiledCamera` has been removed and entirely replaced by the internal `TerminalCamera`. 
- `ToWorld` has been replaced by `TerminalTransform`, which can be used in combination with `TerminalCamera` to transform positions between world space and terminal grid points. See the "transform" example.
- `TerminalBorder` is now a seperate component.
- `TerminalBundle` has been replaced with by bevy's required components system. `TerminalBorder`, `TerminalMeshPivot`, `TerminalMeshTileScaling`, `SetTerminalGridPosition`, and `SetTerminalLayerPosition` are examples of individual components that can added to customize how the terminal gets displayed. See the examples.
- With the bevy color changes all colors are now represented internally as LinearRgba. The `color` module has a list of const lrgba named colors for convenience.
- Probably more that I can't think of.