use std::collections::VecDeque;

use glam::Vec2;
use owned_ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::GlyphBuffer;

use crate::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::{
        components::{
            mixins::{Anchor, AnchorCommand},
            types::TextNode,
        },
        systems::construct_path::text::token::TokenKind,
    },
};

use super::{
    current_line::CurrentLine,
    line_break_strategy::{
        break_on_word::BreakOnWordLineBreakStrategy, LineBreakBehavior, LineBreakStrategy,
        ShouldBreakLine,
    },
    token::Token,
    token_stream::TokenStream,
    token_with_shape::TokenWithShape,
};

pub struct TextBuilder {
    subpaths: Vec<Vec<Anchor>>,
    max_line_width: f32,

    // Current
    // TODO: improve
    current_subpath: Vec<Anchor>,
    current_pos: Vec2,
    current_offset: Vec2,
    current_ascender: f32,
    current_max_ascender: f32,
    current_scale: f32,
}

impl TextBuilder {
    pub fn new(max_line_width: f32) -> Self {
        Self {
            subpaths: Vec::new(),
            max_line_width,
            current_subpath: Vec::new(),
            current_pos: Vec2::ZERO,
            current_offset: Vec2::ZERO,
            current_ascender: 0.0,
            current_max_ascender: 0.0,
            current_scale: 0.0,
        }
    }

    pub fn process_text(&mut self, text: &TextNode, font_cache: &mut FontCacheRes) {
        let mut token_stream = TokenStream::from_text(text, font_cache);
        let lines = token_stream.drain_into_lines();

        for line in lines {
            self.process_line(VecDeque::from(line), &token_stream);
        }
    }

    fn process_line(&mut self, line: VecDeque<Token>, token_stream: &TokenStream) {
        let mut to_process_tokens = line;
        let mut line_break_strategy = BreakOnWordLineBreakStrategy::new();

        let mut current_line = CurrentLine::new(self.max_line_width);
        while let Some(mut token) = to_process_tokens.pop_front() {
            // Endless loop prevention
            token.track_processed();

            // Process Space and TextFragment tokens
            if let TokenKind::Space { style, .. } | TokenKind::TextFragment { style, .. } =
                &token.kind
            {
                if let Some(font_face) = token_stream.get_buzz_face(style.font_id) {
                    let mut token_with_shape = TokenWithShape::new(token, &font_face);

                    // Check for line break requirement
                    match line_break_strategy.should_break(
                        &mut current_line,
                        &mut token_with_shape,
                        to_process_tokens.is_empty(),
                    ) {
                        // Handle line break
                        ShouldBreakLine::True(line_break_behavior) => {
                            // Process the current line
                            self.process_current_line(&mut current_line, token_stream);

                            // Handle line break behavior
                            match line_break_behavior {
                                // Requeue overflown tokens and the current token
                                // to be appended in the next line
                                LineBreakBehavior::AppendOverflownTokens(overflown_tokens) => {
                                    to_process_tokens.push_front(token_with_shape.token);
                                    for overflown_token in overflown_tokens
                                        .into_iter()
                                        .map(|token_with_shape| token_with_shape.token)
                                        .rev()
                                    {
                                        to_process_tokens.push_front(overflown_token);
                                    }
                                }

                                // Requeue current token to be appended in the next line
                                LineBreakBehavior::AppendNextToken => {
                                    to_process_tokens.push_front(token_with_shape.token);
                                }
                                _ => {}
                            }
                        }

                        // Append token to the current line
                        ShouldBreakLine::False => {
                            current_line.append(token_with_shape);
                        }
                    }
                }
            }
        }

        // Process the final line
        self.process_current_line(&mut current_line, token_stream);
    }

    fn process_current_line(&mut self, current_line: &mut CurrentLine, token_stream: &TokenStream) {
        self.move_to_new_line(&current_line);

        // Process current line
        if !current_line.is_empty() {
            for token_with_shape in current_line.drain(..) {
                if let TokenKind::Space { metric, style, .. }
                | TokenKind::TextFragment { metric, style, .. } = token_with_shape.token.kind
                {
                    if let Some(font_face) = token_stream.get_buzz_face(style.font_id) {
                        self.current_scale = metric.font_scale;
                        self.current_ascender = metric.ascender;
                        self.process_glyphs(&token_with_shape.glyph_buffer, font_face);
                    }
                }
            }
        }
    }

    /// Processes the glyphs of a text and constructs their paths.
    fn process_glyphs(&mut self, glyph_buffer: &GlyphBuffer, font_face: &rustybuzz::Face) {
        let baseline_adjustment = self.current_max_ascender - self.current_ascender;

        for (glyph_position, glyph_info) in glyph_buffer
            .glyph_positions()
            .iter()
            .zip(glyph_buffer.glyph_infos())
        {
            // Calculate and set the glyph offset for positioning
            self.current_offset = Vec2::new(
                glyph_position.x_offset as f32,
                glyph_position.y_offset as f32 - baseline_adjustment,
            ) * self.current_scale;

            // Outline the glyph and add it to the current path
            font_face.outline_glyph(GlyphId(glyph_info.glyph_id as u16), self);
            if !self.current_subpath.is_empty() {
                self.subpaths
                    .push(core::mem::replace(&mut self.current_subpath, Vec::new()));
            }

            // Update the position for the next glyph
            self.current_pos += Vec2::new(
                glyph_position.x_advance as f32,
                glyph_position.y_advance as f32,
            ) * self.current_scale;
        }
    }

    /// Moves the current position to the start of a new line.
    fn move_to_new_line(&mut self, current_line: &CurrentLine) {
        let current_line_metric = current_line.compute_line_metric();
        self.current_max_ascender = current_line_metric.max_ascender;
        // To align the line glyphs with Figma behaviour the inital vertical line offset is 0.78 * height
        self.current_pos.y +=
            current_line_metric.height * if self.current_pos.y == 0.0 { 0.78 } else { 1.0 };
        self.current_pos.x = 0.0;
    }

    /// Converts a point from local to global coordinates, scaling accordingly.
    fn point(&self, x: f32, y: f32) -> Vec2 {
        self.current_pos
            + self.current_offset
            + Vec2::new(x, self.current_ascender - y) * self.current_scale
    }

    /// Flushes the current subpath into other subpaths if it's not empty.
    fn flush_current_subpath(&mut self) {
        if !self.current_subpath.is_empty() {
            self.subpaths
                .push(std::mem::take(&mut self.current_subpath));
        }
    }

    /// Converts the constructed paths into a flat vector of vertices.
    pub fn into_vertices(&mut self) -> Vec<Anchor> {
        self.subpaths.drain(..).flatten().collect()
    }
}

impl OutlineBuilder for TextBuilder {
    /// Starts a new subpath at the given point.
    fn move_to(&mut self, x: f32, y: f32) {
        self.flush_current_subpath();
        self.current_subpath.push(Anchor {
            command: AnchorCommand::MoveTo {
                position: self.point(x, y),
            },
        });
    }

    /// Adds a line to the current subpath.
    fn line_to(&mut self, x: f32, y: f32) {
        self.current_subpath.push(Anchor {
            command: AnchorCommand::LineTo {
                position: self.point(x, y),
            },
        });
    }

    /// Converts a quadratic bezier curve to a cubic one and adds it to the current subpath.
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let current_point = self.current_subpath.last().unwrap().get_position().unwrap();
        let control_point = self.point(x1, y1);
        let end_point = self.point(x2, y2);

        // Convert quadratic to cubic bezier control points.
        let cubic_control_point1 = current_point + 2.0 / 3.0 * (control_point - current_point);
        let cubic_control_point2 = end_point + 2.0 / 3.0 * (control_point - end_point);

        self.current_subpath.push(Anchor {
            command: AnchorCommand::CurveTo {
                position: end_point,
                control_point_1: cubic_control_point1,
                control_point_2: cubic_control_point2,
            },
        });
    }

    /// Adds a cubic bezier curve to the current subpath.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.current_subpath.push(Anchor {
            command: AnchorCommand::CurveTo {
                position: self.point(x3, y3),
                control_point_1: self.point(x1, y1),
                control_point_2: self.point(x2, y2),
            },
        });
    }

    /// Closes the current subpath and adds it to other subpaths.
    fn close(&mut self) {
        if let Some(first_anchor) = self.current_subpath.first() {
            if first_anchor.command != AnchorCommand::ClosePath {
                self.current_subpath.push(Anchor {
                    command: AnchorCommand::ClosePath,
                });
            }
        }
        self.flush_current_subpath();
    }
}
