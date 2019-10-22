//! This crate statically embeds the [m5x7 font](https://managore.itch.io/m5x7) by Daniel Linssen.
//! This allows, e.g., text rendering libraries to create examples that don't require an external font dependency.
//!
//! Cache construction with piston2d-graphics:
//! ```
//! # use opengl_graphics::{GlyphCache, TextureSettings};
//! # let texture_settings = TextureSettings::new();
//! let cache = GlyphCache::from_bytes(m5x7::BYTES, (), texture_settings);
//! ```
#![cfg_attr(feature = "parsed", doc = r#"
Direct access with rusttype (requires the `parsed` feature):
```
let glyph = m5x7::FONT.glyph('A');
```"#)]
#![doc(html_root_url = "https://docs.rs/m5x7/1.0.0")]

/// The [m5x7 font](https://managore.itch.io/m5x7) by Daniel Linssen, as TTF bytes
/// ```
/// # fn main() -> std::io::Result<()> {
/// let parsed = rusttype::Font::from_bytes(m5x7::BYTES)?;
/// # Ok(()) }
/// ```
pub const BYTES: &'static [u8] = include_bytes!("m5x7.ttf");

#[cfg(feature = "parsed")]
#[macro_use] extern crate lazy_static;

#[cfg(feature = "parsed")]
lazy_static! {
    /// The [m5x7 font](https://managore.itch.io/m5x7) by Daniel Linssen, as a rusttype::Font
    /// ```
    /// let glyph = m5x7::FONT.glyph('A');
    /// ```
    #[cfg(feature = "parsed")]
    pub static ref FONT: rusttype::Font<'static> = {
        rusttype::Font::from_bytes(BYTES)
            .expect("Error parsing m5x7 font")
    };
}
