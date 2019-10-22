# Daniel Linssen's [m5x7](https://managore.itch.io/m5x7) as a Rust crate

[![docs.rs](https://docs.rs/m5x7/badge.svg)](https://docs.rs/m5x7)

This crate statically embeds the [m5x7 font](https://managore.itch.io/m5x7) by Daniel Linssen.
This allows, e.g., text rendering libraries to create examples that don't require an external font dependency.
This crate, like the m5x7 font, is CC0 licensed, but giving credit to Daniel Linssen for creating the font would be appreciated.

Cache construction with piston2d-graphics:
```rust
let cache = GlyphCache::from_bytes(m5x7::BYTES, (), texture_settings);
```

Direct access with rusttype (requires the `parsed` feature):
```rust
let glyph = m5x7::FONT.glyph('A');
```
