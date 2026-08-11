mod colors;
mod types;

pub use colors::Color;
use taffy::{LengthPercentage, Rect, Size};
pub use types::{
    AlignItems, Dimension, Display, FlexDirection, FontStyle, JustifyContent, Spacing,
};

#[derive(Default)]
pub struct Style {
    pub(crate) layout: taffy::style::Style,
    pub(crate) color: Option<Color>,
    pub(crate) background_color: Option<Color>,
    pub(crate) border_radius: Option<f32>,
    pub(crate) font_size: Option<f32>,
    pub(crate) font_weight: Option<f32>,
    pub(crate) font_style: Option<FontStyle>,
    pub(crate) line_height: Option<f32>,
    pub(crate) tracking: Option<f32>,
}

impl Style {
    pub fn width(mut self, width: Dimension) -> Self {
        self.layout.size.width = width.into();
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        self.layout.size.height = height.into();
        self
    }

    pub fn size(mut self, width: Dimension, height: Dimension) -> Self {
        self.layout.size = taffy::Size {
            width: width.into(),
            height: height.into(),
        };
        self
    }

    pub fn min_width(mut self, width: Dimension) -> Self {
        self.layout.min_size.width = width.into();
        self
    }

    pub fn min_height(mut self, height: Dimension) -> Self {
        self.layout.min_size.height = height.into();
        self
    }

    pub fn display(mut self, display: Display) -> Self {
        self.layout.display = display.into();
        self
    }

    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.layout.flex_direction = direction.into();
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.layout.align_items = Some(align.into());
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.layout.justify_content = Some(justify.into());
        self
    }

    pub fn flex_grow(mut self, grow: f32) -> Self {
        self.layout.flex_grow = grow;
        self
    }

    pub fn flex_shrink(mut self, shrink: f32) -> Self {
        self.layout.flex_shrink = shrink;
        self
    }

    pub fn padding(mut self, padding: Spacing) -> Self {
        let p = padding.into();

        self.layout.padding = Rect {
            left: p,
            right: p,
            top: p,
            bottom: p,
        };
        self
    }

    pub fn padding_axis(mut self, horizontal: Spacing, vertical: Spacing) -> Self {
        let h = horizontal.into();
        let v = vertical.into();

        self.layout.padding = Rect {
            left: h,
            right: h,
            top: v,
            bottom: v,
        };
        self
    }

    pub fn padding_tblr(
        mut self,
        left: Spacing,
        right: Spacing,
        top: Spacing,
        bottom: Spacing,
    ) -> Self {
        self.layout.padding = Rect {
            left: left.into(),
            right: right.into(),
            top: top.into(),
            bottom: bottom.into(),
        };
        self
    }

    pub fn margin(mut self, margin: Spacing) -> Self {
        let m = margin.into();
        self.layout.margin = Rect {
            left: m,
            right: m,
            top: m,
            bottom: m,
        };
        self
    }

    pub fn margin_axis(mut self, horizontal: Spacing, vertical: Spacing) -> Self {
        let h = horizontal.into();

        let v = vertical.into();
        self.layout.margin = Rect {
            left: h,
            right: h,
            top: v,
            bottom: v,
        };
        self
    }

    pub fn margin_tblr(
        mut self,
        left: Spacing,
        right: Spacing,
        top: Spacing,
        bottom: Spacing,
    ) -> Self {
        self.layout.margin = Rect {
            left: left.into(),
            right: right.into(),
            top: top.into(),
            bottom: bottom.into(),
        };
        self
    }

    pub fn gap(mut self, gap: Spacing) -> Self {
        let g = gap.into();

        self.layout.gap = Size {
            width: g,
            height: g,
        };
        self
    }

    pub fn border(mut self, px: f32) -> Self {
        self.layout.border = Rect {
            left: LengthPercentage::length(px),
            right: LengthPercentage::length(px),
            top: LengthPercentage::length(px),
            bottom: LengthPercentage::length(px),
        };
        self
    }

    pub fn border_radius(mut self, px: f32) -> Self {
        self.border_radius = Some(px);
        self
    }

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

    pub fn line_height(mut self, px: f32) -> Self {
        self.line_height = Some(px);
        self
    }

    pub fn tracking(mut self, px: f32) -> Self {
        self.tracking = Some(px);
        self
    }
}
