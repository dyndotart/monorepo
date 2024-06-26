use dyn_fonts_book::font::FontId;
use dyn_utils::units::em::Em;
use rustybuzz::ttf_parser::GlyphId;
use std::ops::Range;

#[derive(Debug, Default, Clone)]
pub struct Glyph {
    /// A font id this glyph belongs to.
    pub font_id: FontId,
    /// A glyph id.
    /// If the glyph id equals 0, the glyph is missing.
    pub glyph_id: GlyphId,
    /// Codepoint in the orignal string.
    pub codepoint: char,
    /// Position in bytes in the original string.
    pub range: Range<usize>,
    /// A width relative the font size.
    ///
    /// It's different from advance in that it's not affected by letter spacing, ..
    pub width: Em,
    /// An advance in horizontal direction relative the font size.
    pub x_advance: Em,
    /// An advance in vertical direction relative the font size.
    pub y_advance: Em,
    /// An offset in horizontal direction relative the font size.
    pub x_offset: Em,
    /// An offset in vertical direction relative the font size.
    pub y_offset: Em,
    /// The distance from the baseline to the typographic ascender
    /// relative the font size.
    pub ascent: Em,
    /// The distance from the baseline to the typographic descender
    /// relative the font size.
    pub descent: Em,
}

impl Glyph {
    pub fn height(&self) -> Em {
        self.ascent - self.descent
    }

    pub fn is_missing(&self) -> bool {
        self.glyph_id.0 == 0
    }
}
