# Changelog

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