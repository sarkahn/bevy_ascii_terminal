# Changelog
## [0.8.1]
### Changed
* Inverted the y axis. Now y0 == the bottom and height 0 - 1 == the top line. This will make it simpler to translate from world
    positions to terminal positions.

### Added
* `draw_horizontal_bar` functions.

## [0.8.0] - 01-08-2022

### Changed
* Added a changelog!
* Updated to bevy 0.6
* Fonts changed to bevy assets (`Handle<Image>`). Built in fonts can be retrieved via the `BuiltInFontHandles` resource.
* All functions taking tuples `(i32,i32)`/`(u32,u32)` now take array positions instead: `[i32;2]`/`[u32;2]`.
* `MeshPipeline` was removed, the renderer now uses the new high level [SpecializedMaterial](https://docs.rs/bevy_pbr/0.6.0/bevy_pbr/trait.SpecializedMaterial.html) abstraction.
* `GlyphMapping` has been changed to the more generalized `UVMapping`, which maps glyphs directly to uvs. This should make drawing graphics with a terminal more ergonomic and pave the way for things like Tiled/LDTK support.
* Functionality for translating to and from cp437/texture indices has been moved to [code_page_437.rs](src/renderer/code_page_437.rs) file.

### Removed
* The "Font" type has been removed, instead fonts are now just bevy textures (`Image`). To change fonts you must assign a `Handle<Image>` to the `TerminalMaterial`'s texture field.
* Terminal systems now run in the default stage.