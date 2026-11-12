use cosmic_text::Buffer;

use crate::{signal::Value, style::Style};

#[non_exhaustive]
#[allow(dead_code)]
pub enum Type {
    //display
    Text {
        text: Value<String>,
    },
    Link {
        src: Value<String>,
    },
    Image {
        src: Value<String>,
    },
    Progress {
        value: Value<f32>,
    },
    Separator,
    Svg {
        src: Value<String>,
    },

    //layout
    Panel,
    Modal,
    Column,
    Row,
    Grid {
        cols: u8,
        rows: u8,
    },
    Scrollarea,

    //interactive
    Button {
        text: Value<String>,
    },
    ImageButton,
    Checkbox {
        checked: Value<bool>,
    },
    RadioButton {
        checked: Value<bool>,
    },
    Select {
        selected: Value<String>,
        options: Vec<String>,
    },
    Slider {
        value: Value<f32>,
    },
    DragValue {
        value: Value<f32>,
    },
    Input {
        value: Value<String>,
    },
    Switch {
        toggle: Value<bool>,
    },

    //special
    Menu,
    Accordion,

    // Graphs and custom rendering
    Node,
    Canvas,

    //color
    ColorWheel,
    ColorPad,
    Hue,
    Lightness,
    Alpha,
}

pub struct Widget {
    pub type_of: Type,
    pub children: Vec<Widget>,
    pub styles: Style,
    pub buffer: Option<Buffer>,
}

impl Widget {
    pub fn child(mut self, child: Widget) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<Widget>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn styles(mut self, styles: Style) -> Self {
        self.styles = styles;
        self
    }

    pub fn on_click() {}

    pub fn on_hover() {}

    pub fn on_change() {}

    pub fn on_press() {}

    pub fn on_release() {}

    pub fn on_key_down() {}

    pub fn on_key_up() {}

    pub fn on_focus() {}

    pub fn on_blur() {}
}
