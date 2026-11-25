use std::sync::Arc;

use parley::{
    FontContext, FontWeight, Layout, LayoutContext,
    StyleProperty::{self},
    fontique::Blob,
};

use crate::{
    style,
    text::atlas::Atlas,
    widgets::{Widget, WidgetType},
};

pub mod atlas;
mod editor;

#[derive(Clone, Copy, Debug)]
pub struct CachedGlyph {
    pub rect: [u32; 4],
    pub offset: [i32; 2],
}

pub struct Text {
    pub font: FontContext,
    pub layout: LayoutContext<[f32; 4]>,
    pub atlas: Atlas,
}

impl Text {
    pub(crate) fn new() -> Self {
        let mut font = FontContext::new();

        let regular = include_bytes!("../font/Inter-Variable.ttf");
        let italic = include_bytes!("../font/Inter-Italic.ttf");

        let blob_r: Blob<u8> = Blob::new(Arc::new(regular));

        let blob_i: Blob<u8> = Blob::new(Arc::new(italic));

        let _ = font.collection.register_fonts(blob_r, None);

        let _ = font.collection.register_fonts(blob_i, None);

        Self {
            font,
            layout: LayoutContext::new(),
            atlas: Atlas::new(),
        }
    }

    pub(crate) fn shape(&mut self, widget: &Widget, dpi: f32) -> Layout<[f32; 4]> {
        let mut layout: Option<Layout<[f32; 4]>> = None;

        match &widget.type_of {
            WidgetType::Text { text, font_name } => {
                let text = text.get();

                layout = Some(self.build(widget, text, font_name, dpi));
            }

            WidgetType::Link { label } => {
                let text = label.get();
                layout = Some(self.build(widget, text, &None, dpi));
            }

            WidgetType::Button { text } => {
                let text = text.get();

                layout = Some(self.build(widget, text, &None, dpi));
            }

            _ => {}
        };

        layout.unwrap()
    }

    fn build(
        &mut self,
        widget: &Widget,
        text: String,
        font_name: &Option<String>,
        dpi: f32,
    ) -> Layout<[f32; 4]> {
        let mut builder = self.layout.ranged_builder(&mut self.font, &text, dpi, true);

        let weight: f32 = match widget.styles.font_weight {
            style::FontWeight::Thin => 100.0,
            style::FontWeight::ExtraLight => 200.0,
            style::FontWeight::Light => 300.0,
            style::FontWeight::Normal => 400.0,
            style::FontWeight::SemiBold => 500.0,
            style::FontWeight::Bold => 600.0,
            style::FontWeight::ExtraBold => 700.0,
        };

        let font_style = match widget.styles.font_style {
            style::FontStyle::Normal => parley::FontStyle::Normal,
            style::FontStyle::Italic => parley::FontStyle::Italic,
        };

        let name = match font_name {
            Some(name) => name,
            None => "Inter",
        };

        let color = widget.styles.color;

        builder.push_default(StyleProperty::FontFamily(parley::FontFamily::Source(
            std::borrow::Cow::Borrowed(name),
        )));

        builder.push_default(StyleProperty::FontSize(widget.styles.font_size));

        builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));

        builder.push_default(StyleProperty::FontStyle(font_style));

        builder.push_default(StyleProperty::LineHeight(parley::LineHeight::Absolute(
            widget.styles.line_height,
        )));

        builder.push_default(StyleProperty::Brush([color.r, color.g, color.b, color.a]));

        builder.push_default(StyleProperty::Underline(widget.styles.underline));

        builder.push_default(StyleProperty::Strikethrough(widget.styles.strike_through));

        builder.build(&text)
    }
}
