use super::{glyph::GlyphToken, ShapeBuffer, ShapeToken};
use crate::{shape::shape_text_with_fallback, text_attrs::TextAttrs};
use dyn_fonts_book::FontsBook;
use dyn_utils::units::abs::Abs;
use std::ops::Range;

/// Groups glyphs into a continuous fragment of text, typically a word or number.
#[derive(Debug, Clone)]
pub struct TextFragmentToken {
    range: Range<usize>,
    /// Glyph tokens that make up the text fragment.
    tokens: Vec<GlyphToken>,
}

impl TextFragmentToken {
    pub fn from_text(
        text: &str,
        range: Range<usize>,
        attrs: &TextAttrs,
        shape_buffer: &mut ShapeBuffer,
        fonts_book: &mut FontsBook,
    ) -> Self {
        let mut tokens: Vec<GlyphToken> = Vec::with_capacity(range.len());

        log::info!(
            "TextFragmentToken for text: '{}' ({:?})",
            &text[range.clone()],
            range
        );

        if let Some(font) = fonts_book.get_font_by_info(attrs.get_font_info()) {
            let (glyphs, buffer) = shape_text_with_fallback(
                text,
                range.clone(),
                shape_buffer.buffer.take().unwrap_or_default(),
                &font,
                fonts_book,
            );
            shape_buffer.buffer = Some(buffer);
            tokens.extend(
                glyphs
                    .into_iter()
                    .map(|glyph| GlyphToken::new(glyph, attrs.get_font_size())),
            );
        }

        return Self { range, tokens };
    }

    pub fn get_tokens(&self) -> &Vec<GlyphToken> {
        &self.tokens
    }

    pub(crate) fn get_tokens_mut(&mut self) -> &mut Vec<GlyphToken> {
        &mut self.tokens
    }
}

impl ShapeToken for TextFragmentToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    fn x_advance(&self) -> Abs {
        self.tokens
            .iter()
            .fold(Abs::zero(), |acc, token| acc + token.layout.x_advance)
    }

    fn y_advance(&self) -> Abs {
        self.tokens
            .iter()
            .fold(Abs::zero(), |acc, token| acc + token.layout.y_advance)
    }
}
