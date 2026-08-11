use std::rc::Rc;

use crate::{events::Event, signal::Value};

#[non_exhaustive]
pub enum WidgetType {
    Panel,
    ScrollArea,
    Tabs {
        active: Value<u8>,
        onchange: Rc<dyn Fn(u8)>,
    },
    Tab {
        label: Value<String>,
    },
    Collapsible {
        expand: Value<bool>,
    },
    Splitter,
    Window,

    Button {
        onclick: Box<dyn Fn()>,
        onhover: Box<dyn Fn()>,
    },
    Checkbox {
        checked: Value<bool>,
        ontoggle: Box<dyn Fn()>,
    },
    Switch {
        checked: Value<bool>,
        ontoggle: Box<dyn Fn()>,
    },
    RadioButton {
        selected: Value<bool>,
        onchange: Box<dyn Fn()>,
    },
    Slider {
        value: Value<f64>,
        min: f64,
        max: f64,
        onchange: Box<dyn Fn()>,
    },
    DragValue {
        value: Value<f64>,
        min: f64,
        max: f64,
        onchange: Box<dyn Fn()>,
    },
    TextInput {
        value: Value<String>,
        onchange: Box<dyn Fn(Event)>,
    },

    Select {
        default: Value<String>,
        options: Vec<String>,
        onchange: Box<dyn Fn()>,
    },

    Text {
        text: Value<String>,
    },

    Icon {
        source: Value<String>,
    },

    Image {
        source: Value<String>,
    },

    ProgressBar {
        value: Value<f64>,
        min: f64,
        max: f64,
    },

    Hyperlink {
        label: Value<String>,
        onclick: Box<dyn Fn()>,
    },

    Separator,

    Canvas,
}
