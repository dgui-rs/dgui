mod color;
mod types;

pub use color::Color;
use taffy::{LengthPercentage, Rect, Size};
pub use types::{
    AlignItems, Dimension, Display, FlexDirection, FontStyle, FontWeight, JustifyContent, Spacing,
};

pub struct Style {
    pub(crate) layout: taffy::style::Style,
    pub(crate) color: Color,
    pub(crate) background_color: Color,
    pub(crate) border_color: Color,
    pub(crate) border_radius: f32,
    pub(crate) font_size: f32,
    pub(crate) font_weight: FontWeight,
    pub(crate) font_style: FontStyle,
    pub(crate) line_height: f32,
    pub(crate) letter_spacing: f32,
    pub(crate) underline: bool,
    pub(crate) strike_through: bool,
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
        self.layout.align_items = align.into();
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.layout.justify_content = justify.into();
        self
    }

    pub fn flex_grow(mut self) -> Self {
        self.layout.flex_grow = 1.0;
        self
    }

    pub fn flex_shrink(mut self) -> Self {
        self.layout.flex_shrink = 1.0;
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

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn border_radius(mut self, px: f32) -> Self {
        self.border_radius = px;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    pub fn line_height(mut self, px: f32) -> Self {
        self.line_height = px;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn strike_through(mut self) -> Self {
        self.strike_through = true;
        self
    }

    pub fn letter_spacing(mut self, spacing: f32) -> Self {
        self.letter_spacing = spacing;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            layout: Default::default(),
            color: Color::WHITE,
            background_color: Color::NEUTRAL_950,
            border_color: Color::TRANSPARENT,
            border_radius: 0.0,
            font_size: 14.0,
            font_weight: FontWeight::Normal,
            font_style: FontStyle::Normal,
            letter_spacing: 0.0,
            line_height: 1.2,
            underline: false,
            strike_through: false,
        }
    }
}
