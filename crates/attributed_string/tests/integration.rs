mod common;

use common::assert_attributed_string_rendered;
use dyn_attributed_string::{
    attrs::{Attrs, AttrsInterval},
    AttributedString, AttributedStringConfig, LineWrap,
};
use dyn_fonts_book::font::info::FontFamily;
use dyn_utils::{
    properties::size::Size,
    units::{abs::Abs, font_unit::FontUnit},
};

#[test]
fn test_hebrew_word() {
    let text = String::from("בדיקה");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(120.0), Abs::pt(60.0)),
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("a_hebrew_word", &mut attributed_string);
}

#[test]
fn test_hebrew_paragraph() {
    let text = String::from("השועל החום המהיר קופץ מעל הכלב העצלן");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(400.0), Abs::pt(110.0)),
        line_wrap: LineWrap::Word,
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("a_hebrew_paragraph", &mut attributed_string);
}

#[test]
fn test_english_mixed_with_hebrew_paragraph() {
    let text = String::from("Many computer programs fail to display bidirectional text correctly. For example, this page is mostly LTR English script, and here is the RTL Hebrew name Sarah: שרה, spelled sin (ש) on the right, resh (ר) in the middle, and heh (ה) on the left.");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(16.0))
            .line_height(FontUnit::abs(Abs::pt(20.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(400.0), Abs::pt(120.0)),
        line_wrap: LineWrap::Word,
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("some_english_mixed_with_hebrew", &mut attributed_string);
}

#[test]
fn test_arabic_word() {
    let text = String::from("خالصة");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(120.0), Abs::pt(60.0)),
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("an_arabic_word", &mut attributed_string);
}

#[test]
fn test_arabic_paragraph() {
    let text = String::from("الثعلب البني السريع يقفز فوق الكلب الكسول");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(400.0), Abs::pt(110.0)),
        line_wrap: LineWrap::Word,
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("an_arabic_paragraph", &mut attributed_string);
}

#[test]
fn test_english_mixed_with_arabic_paragraph() {
    let text = String::from("I like to render اللغة العربية in Rust!");
    let attrs_intervals = vec![AttrsInterval {
        start: 0,
        stop: text.len(),
        val: Attrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];
    let config = AttributedStringConfig {
        size: Size::new(Abs::pt(400.0), Abs::pt(110.0)),
        line_wrap: LineWrap::Word,
        ..Default::default()
    };

    let mut attributed_string = AttributedString::new(text, attrs_intervals, config);

    assert_attributed_string_rendered("some_english_mixed_with_arabic", &mut attributed_string);
}
