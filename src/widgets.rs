use std::cell::Cell;

use taffy::NodeId;

use crate::{
    signal::{TextValue, Value},
    style::Style,
};

#[non_exhaustive]
pub enum WidgetType {
    //display
    Text {
        text: Box<dyn TextValue>,
        font_name: Option<String>,
    },
    Link {
        label: Value<String>,
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
    Clickable,
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
    Node {
        title: Value<String>,
    },
    Canvas,

    //color
    ColorWheel,
    ColorPad,
    Hue,
    Lightness,
    Alpha,
}

pub struct Widget {
    pub type_of: WidgetType,
    pub children: Vec<Widget>,
    pub styles: Style,
    pub node_id: Cell<Option<NodeId>>,
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

    pub fn add_input_pin() {}

    pub fn add_output_pin() {}
}

#[derive(Default)]
pub struct Pin {
    title: String,
    connection_id: String,
}

impl Pin {
    pub fn connect() {}
    pub fn disconnect() {}
}
