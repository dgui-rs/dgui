use crate::styles::colors::Color;

pub mod colors;

pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Default)]
pub struct Style {
    pub(crate) _layout: taffy::style::Style,
    pub(crate) color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) font_size: Option<f32>,
    pub(crate) font_weight: Option<f32>,
    pub(crate) font_style: Option<FontStyle>,
}

impl Style {
    pub fn color(&mut self, color: Color) {
        self.color = Some(color)
    }

    pub fn background_color(&mut self, color: Color) {
        self.background_color = Some(color)
    }

    pub fn font_size(&mut self, size: f32) {
        self.font_size = Some(size)
    }

    pub fn font_weight(&mut self, weight: f32) {
        self.font_weight = Some(weight)
    }

    pub fn font_style(&mut self, style: FontStyle) {
        self.font_style = Some(style)
    }
}
