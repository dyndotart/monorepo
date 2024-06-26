mod common;

use common::assert_attributed_string_rendered;
use dyn_attributed_string::{
    layout::{
        layouter::LayouterConfig, HorizontalTextAlignment, LayoutSize, LineWrap,
        VerticalTextAlignment,
    },
    text_attrs::{TextAttrs, TextAttrsInterval},
    AttributedString,
};
use dyn_fonts_book::font::{info::FontFamily, variant::FontWeight};
use dyn_utils::units::{abs::Abs, auto_length::AutoLength, em::Em, font_unit::FontUnit};

#[test]
fn test_hebrew_word() {
    let text = String::from("בדיקה");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_hebrew_word",
        &mut attributed_string,
        LayouterConfig {
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(120.0)),
                AutoLength::abs(Abs::pt(60.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_hebrew_paragraph() {
    let text = String::from("השועל החום המהיר קופץ מעל הכלב העצלן");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_hebrew_paragraph",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(400.0)),
                AutoLength::abs(Abs::pt(110.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_english_mixed_with_hebrew_paragraph() {
    let text = String::from("Many computer programs fail to display bidirectional text correctly. For example, this page is mostly LTR English script, and here is the RTL Hebrew name Sarah: שרה, spelled sin (ש) on the right, resh (ר) in the middle, and heh (ה) on the left.");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(16.0))
            .line_height(FontUnit::abs(Abs::pt(20.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_english_mixed_with_hebrew_paragraph",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(400.0)),
                AutoLength::abs(Abs::pt(120.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_arabic_word() {
    let text = String::from("خالصة");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_arabic_word",
        &mut attributed_string,
        LayouterConfig {
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(120.0)),
                AutoLength::abs(Abs::pt(60.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_arabic_paragraph() {
    let text = String::from("الثعلب البني السريع يقفز فوق الكلب الكسول");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_arabic_paragraph",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(400.0)),
                AutoLength::abs(Abs::pt(110.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_english_mixed_with_arabic_paragraph() {
    let text = String::from("I like to render اللغة العربية in Rust!");
    let attrs_intervals = vec![TextAttrsInterval {
        start: 0,
        stop: text.len(),
        val: TextAttrs::new()
            .font_family(FontFamily::Named(String::from("Noto Sans")))
            .font_size(Abs::pt(36.0))
            .line_height(FontUnit::abs(Abs::pt(40.0))),
    }];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_english_mixed_with_arabic_paragraph",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(400.0)),
                AutoLength::abs(Abs::pt(110.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_english_mixed_with_hebrew_paragraph_and_different_fonts() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_english_mixed_with_hebrew_paragraph_and_different_fonts",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_horizontal_text_align_start() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_horizontal_text_align_start",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            horizontal_text_alignment: HorizontalTextAlignment::Start,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_horizontal_text_align_end() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_horizontal_text_align_end",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            horizontal_text_alignment: HorizontalTextAlignment::End,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_horizontal_text_align_left() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_horizontal_text_align_left",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            horizontal_text_alignment: HorizontalTextAlignment::Left,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_horizontal_text_align_right() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_horizontal_text_align_right",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            horizontal_text_alignment: HorizontalTextAlignment::Right,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_horizontal_text_align_center() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_horizontal_text_align_center",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            horizontal_text_alignment: HorizontalTextAlignment::Center,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(240.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_vertical_text_align_top() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_vertical_text_align_top",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            vertical_text_alignment: VerticalTextAlignment::Top,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(300.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_vertical_text_align_bottom() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_vertical_text_align_bottom",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            vertical_text_alignment: VerticalTextAlignment::Bottom,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(300.0)),
            ),
            ..Default::default()
        },
    );
}

#[test]
fn test_vertical_text_align_center() {
    let text = String::from("Hello, world!\nשלום עולם!\nThis is a mix of English and Hebrew.");
    let attrs_intervals = vec![
        TextAttrsInterval {
            start: 0,
            stop: 10,
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Fira Mono")))
                .font_weight(FontWeight::MEDIUM)
                .font_size(Abs::pt(48.0))
                .letter_spacing(FontUnit::em(Em::new(0.5))),
        },
        TextAttrsInterval {
            start: 10,
            stop: text.len(),
            val: TextAttrs::new()
                .font_family(FontFamily::Named(String::from("Noto Sans")))
                .font_weight(FontWeight::REGULAR)
                .font_size(Abs::pt(24.0)),
        },
    ];

    let mut attributed_string = AttributedString::new(text, attrs_intervals);

    assert_attributed_string_rendered(
        "test_vertical_text_align_center",
        &mut attributed_string,
        LayouterConfig {
            line_wrap: LineWrap::Word,
            vertical_text_alignment: VerticalTextAlignment::Center,
            size: LayoutSize::new(
                AutoLength::abs(Abs::pt(300.0)),
                AutoLength::abs(Abs::pt(300.0)),
            ),
            ..Default::default()
        },
    );
}
