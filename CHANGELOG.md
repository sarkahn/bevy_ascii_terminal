# Changelog

## [0.16] - 2024/07/09

### Changed
- `put_string` now aligns all strings to the top-left by default. You can manually specify a `Pivot` to override this.
- `put_string` in addition to wrapping on newlines, will now word wrap by default. You can override this by using the `StringFormatter` trait.  
- `ToWorld` has been replaced by `TerminalTransform`, which can be used to transform positions between world space and terminal grid points. See the "transform" example. 
- `TerminalBundle` has been replaced with by bevy's required components system. TerminalBorder, TerminalMeshPivot and TerminalMeshTileScaling can be customized directly though their respective components when first building your entity. See the examples.