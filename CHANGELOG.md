# Changelog
## [0.9.0]

### Changed
* Removed put_x_formatted functions, replaced in favor of traits on the put_char/put_string functions.
* Points can now be directly specified with a pivot via the point2d trait. Currently only put_char/put_string and clear_box functions will respect the pivot.

## [0.8.1]

### Changed
* Inverted the y axis. Now y0 == the bottom of the terminla and height 0 - 1 == the top line of the terminal. This will make it simpler to translate from world to terminal positions. You can use the format functions described below to write relative to a certain pivot on the terminal.
* Replaced "put\_\*\_color" functions with "put\_\*\_formatted". These formatting functions let you pass in a format object which specifies colors and a pivot to draw from. Whatever position you pass will be relative to the given pivot.

### Removed
* Removed `TileColor` in favor of Bevy's built in colors.
* ColorBlend disabled for now.

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