use super::shape::{
    glyph::GlyphToken, linebreak::LinebreakToken, text_fragment::TextFragmentToken,
    word_separator::WordSeparatorToken, ShapeBuffer, ShapeToken, ShapeTokenVariant,
};
use crate::{attrs::Attrs, utils::is_range_within};
use dyn_fonts_book::FontsBook;
use std::ops::Range;
use unicode_linebreak::BreakClass;

/// A span of text with common attributes.
/// It is guranteed that a span only matches one attribute set.
#[derive(Debug, Clone)]
pub struct SpanToken {
    range: Range<usize>,
    attrs_index: usize,
    /// Shape tokens within the span, including glyphs, words, and separators.
    tokens: Vec<ShapeTokenVariant>,
    /// Bidi level for handling text directionality within the span.
    level: unicode_bidi::Level,
}

impl SpanToken {
    pub fn from_text(
        text: &str,
        range: Range<usize>,
        level: unicode_bidi::Level,
        attrs_index: usize,
        attrs: &Attrs,
        fonts_book: &mut FontsBook,
    ) -> Self {
        let mut tokens: Vec<ShapeTokenVariant> = Vec::new();
        let span_text = &text[range.clone()];
        let mut shape_buffer = ShapeBuffer {
            buffer: Some(rustybuzz::UnicodeBuffer::new()),
        };

        log::info!("SpanToken for text: '{}' ({:?})", span_text, range);

        // Process each character for potential tokenization within the paragraph
        let mut start = range.start;
        for (index, _char) in span_text.char_indices() {
            let global_index = range.start + index; // Adjust index relative to the entire text
            let break_class = unicode_linebreak::break_property(_char as u32);

            match break_class {
                // Handle line break
                BreakClass::Mandatory
                | BreakClass::LineFeed
                | BreakClass::NextLine
                | BreakClass::CarriageReturn => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                attrs,
                                &mut shape_buffer,
                                fonts_book,
                            ),
                        ));
                    }

                    // Add line break token
                    tokens.push(ShapeTokenVariant::Linebreak(LinebreakToken::new(
                        global_index..global_index + 1,
                    )));
                    start = global_index + 1;
                }

                // Handle text segment separation
                BreakClass::Space | BreakClass::ZeroWidthSpace => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                attrs,
                                &mut shape_buffer,
                                fonts_book,
                            ),
                        ));
                    }

                    // Add word separator token
                    tokens.push(ShapeTokenVariant::WordSeparator(
                        WordSeparatorToken::from_text(
                            text,
                            global_index..global_index + 1,
                            attrs,
                            &mut shape_buffer,
                            fonts_book,
                        ),
                    ));
                    start = global_index + 1;
                }
                _ => {}
            }
        }

        // Handle the last text fragment within the paragraph, if any
        if start < range.end {
            tokens.push(ShapeTokenVariant::TextFragment(
                TextFragmentToken::from_text(
                    text,
                    start..range.end,
                    attrs,
                    &mut shape_buffer,
                    fonts_book,
                ),
            ));
        }

        return Self {
            range,
            attrs_index,
            tokens,
            level,
        };
    }

    #[inline]
    pub fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline]
    pub fn get_attrs_index(&self) -> usize {
        self.attrs_index
    }

    #[inline]
    pub fn get_tokens(&self) -> &Vec<ShapeTokenVariant> {
        &self.tokens
    }

    #[inline]
    pub fn get_level(&self) -> &unicode_bidi::Level {
        &self.level
    }

    pub fn iter_glyphs<'a>(&'a self) -> impl Iterator<Item = &'a GlyphToken> + 'a {
        self.tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::WordSeparator(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
            })
    }

    pub(crate) fn iter_glyphs_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.tokens
            .iter_mut()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                ShapeTokenVariant::WordSeparator(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                _ => Box::new(std::iter::empty())
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
            })
    }

    pub(crate) fn iter_glyphs_in_range_mut<'a>(
        &'a mut self,
        range: &'a Range<usize>,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.tokens
            .iter_mut()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                ShapeTokenVariant::WordSeparator(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                _ => Box::new(std::iter::empty())
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
            })
            .filter(move |glyph| is_range_within(glyph.get_range(), &range))
    }

    /// An iterator over glyph clusters within the span.
    ///
    /// This iterator groups adjacent glyphs based on their starting position (byte index),
    /// considering glyphs with the same `start` value as part of the same cluster.
    /// It's particularly useful for processing text where multiple glyphs
    /// contribute to a single visual character (grapheme) or are otherwise logically grouped.
    ///
    /// # Example
    ///
    /// Given glyphs with starting positions like: 0, 2, 2, 2, 3, 4, 4, 5, 5,
    /// the iterator will produce clusters with indices: [0, 1], [1, 4], [4, 5], [5, 7], [7, 9]
    pub fn iter_glyph_clusters<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Vec<&'a GlyphToken>, usize)> + 'a {
        GlyphClusterIterator::new(&self.tokens)
    }
}

struct GlyphClusterIterator<'a> {
    glyphs: Vec<&'a GlyphToken>,
    index: usize,
}

impl<'a> GlyphClusterIterator<'a> {
    fn new(tokens: &'a [ShapeTokenVariant]) -> Self {
        let glyphs: Vec<&GlyphToken> = tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => vec![token],
                ShapeTokenVariant::TextFragment(token) => token.get_tokens().iter().collect(),
                ShapeTokenVariant::WordSeparator(token) => token.get_tokens().iter().collect(),
                _ => Vec::new(),
            })
            .collect();

        Self { glyphs, index: 0 }
    }
}

impl<'a> Iterator for GlyphClusterIterator<'a> {
    type Item = (Vec<&'a GlyphToken>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.glyphs.len() {
            return None;
        }

        let mut cluster = Vec::new();
        let cluster_start = self.glyphs[self.index].get_range().start;

        // Iterate through the glyphs, and collect glyphs
        // that belong to the current cluster (having the same byte index and thus `start` value)
        while self.index < self.glyphs.len()
            && self.glyphs[self.index].get_range().start == cluster_start
        {
            cluster.push(self.glyphs[self.index]);
            self.index += 1;
        }

        if !cluster.is_empty() {
            Some((cluster, cluster_start))
        } else {
            None
        }
    }
}
