pub enum FontStyle {
    Normal,
    Italic,
}

pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
}

pub enum Spacing {
    Px(f32),
    Percent(f32),
}

pub enum Display {
    Block,
    Flex,
    Grid,
    None,
}

pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl From<FlexDirection> for taffy::FlexDirection {
    fn from(dir: FlexDirection) -> Self {
        match dir {
            FlexDirection::Row => taffy::FlexDirection::Row,
            FlexDirection::Column => taffy::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::FlexDirection::ColumnReverse,
        }
    }
}

impl From<AlignItems> for taffy::AlignItems {
    fn from(a: AlignItems) -> Self {
        match a {
            AlignItems::Start => taffy::AlignItems::START,
            AlignItems::Center => taffy::AlignItems::CENTER,
            AlignItems::End => taffy::AlignItems::END,
            AlignItems::Baseline => taffy::AlignItems::BASELINE,
            AlignItems::Stretch => taffy::AlignItems::STRETCH,
        }
    }
}

impl From<Dimension> for taffy::Dimension {
    fn from(dim: Dimension) -> Self {
        match dim {
            Dimension::Auto => taffy::Dimension::auto(),
            Dimension::Px(v) => taffy::Dimension::length(v),
            Dimension::Percent(v) => taffy::Dimension::percent(v),
        }
    }
}

impl From<Display> for taffy::Display {
    fn from(display: Display) -> Self {
        match display {
            Display::Block => taffy::Display::Block,
            Display::Flex => taffy::Display::Flex,
            Display::Grid => taffy::Display::Grid,
            Display::None => taffy::Display::None,
        }
    }
}

impl From<JustifyContent> for taffy::JustifyContent {
    fn from(justify: JustifyContent) -> Self {
        match justify {
            JustifyContent::FlexStart => taffy::JustifyContent::FLEX_START,
            JustifyContent::Center => taffy::JustifyContent::CENTER,
            JustifyContent::FlexEnd => taffy::JustifyContent::FLEX_END,
            JustifyContent::SpaceAround => taffy::JustifyContent::SPACE_AROUND,
            JustifyContent::SpaceBetween => taffy::JustifyContent::SPACE_BETWEEN,
            JustifyContent::SpaceEvenly => taffy::JustifyContent::SPACE_EVENLY,
        }
    }
}

impl From<Spacing> for taffy::LengthPercentage {
    fn from(s: Spacing) -> Self {
        match s {
            Spacing::Px(v) => taffy::LengthPercentage::length(v),
            Spacing::Percent(v) => taffy::LengthPercentage::percent(v),
        }
    }
}

impl From<Spacing> for taffy::LengthPercentageAuto {
    fn from(s: Spacing) -> Self {
        match s {
            Spacing::Px(v) => taffy::LengthPercentageAuto::length(v),
            Spacing::Percent(v) => taffy::LengthPercentageAuto::percent(v),
        }
    }
}
