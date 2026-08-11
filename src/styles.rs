mod colors;
mod types;

pub use colors::Color;
pub use types::{Dimension, FontStyle};

#[derive(Default)]
pub struct Style {
    pub(crate) layout: taffy::style::Style,
    pub(crate) color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) font_size: Option<f32>,
    pub(crate) font_weight: Option<f32>,
    pub(crate) font_style: Option<FontStyle>,
}

impl Style {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn font_weight(mut self, weight: f32) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.font_style = Some(style);
        self
    }
}
