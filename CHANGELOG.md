# Changelog

## [0.16] - 2025/02/17

### Changes
- `put_string` now aligns all strings to the top-left by default. You can manually specify a `Pivot` to override this. In addition to wrapping on newlines, it will now word wrap by default. You can override this with the `dont_word_wrap` function.
- Dependency on `TiledCamera` has been removed and entirely replaced by the internal `TerminalCamera`. 
- `ToWorld` has been replaced by `TerminalTransform`, which can be used in combination with `TerminalCamera` to transform positions between world space and terminal grid points. See the "transform" example.
- `TerminalBorder` is now a seperate component.
- `TerminalBundle` has been replaced with by bevy's required components system. `TerminalBorder`, `TerminalMeshPivot`, `TerminalMeshTileScaling`, `SetTerminalGridPosition`, and `SetTerminalLayerPosition` are examples of individual components that can added to customize how the terminal gets displayed. See the examples.
- With the bevy color changes all colors are now represented internally as LinearRgba. The `color` module has a list of const lrgba named colors for convenience.
- Probably more that I can't think of.