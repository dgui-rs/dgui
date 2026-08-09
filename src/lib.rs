use std::cell::Cell;

use thiserror::Error;

use crate::{
    events::Event,
    signal::{Flags, Value},
    styles::Style,
    widgets::WidgetType,
};

mod events;
pub mod layout;
pub mod signal;
pub mod styles;
mod text;
mod widgets;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

thread_local! {
    static DIRTY: Cell<Flags> = const { Cell::new(Flags::UNSIGNALED) };
}

pub struct Widget {
    pub type_of: WidgetType,
    pub children: Option<Vec<Widget>>,
    pub style: Style,
}

impl Widget {
    pub fn panel(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Panel,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn scrollarea(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::ScrollArea,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn tabs(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Tabs,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn tab(children: Vec<Widget>, label: String) -> Self {
        Self {
            type_of: WidgetType::Tab { label },
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn collapsible<F>(
        children: Vec<Widget>,
        expand: impl Into<Value<bool>>,
        ontoggle: F,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Collapsible {
                expand: expand.into(),
                ontoggle: Box::new(ontoggle),
            },
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn splitter(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Splitter,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn window(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Window,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn button<F1, F2>(children: Vec<Widget>, onclick: F1, onhover: F2) -> Self
    where
        F1: Fn() + 'static,
        F2: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Button {
                onclick: Box::new(onclick),
                onhover: Box::new(onhover),
            },
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn switch<F>(value: impl Into<Value<bool>>, ontoggle: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Switch {
                checked: value.into(),
                ontoggle: Box::new(ontoggle),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn checkbox<F>(value: impl Into<Value<bool>>, ontoggle: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Checkbox {
                checked: value.into(),
                ontoggle: Box::new(ontoggle),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn radio_button<F>(
        label: impl Into<String>,
        value: impl Into<Value<bool>>,
        onchange: F,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::RadioButton {
                label: label.into(),
                selected: value.into(),
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn slider<F>(value: impl Into<Value<f64>>, min: f64, max: f64, onchange: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Slider {
                value: value.into(),
                min,
                max,
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn drag_value<F>(value: impl Into<Value<f64>>, min: f64, max: f64, onchange: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::DragValue {
                value: value.into(),
                min,
                max,
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn text_input<F>(value: impl Into<Value<String>>, onchange: F) -> Self
    where
        F: Fn(Event) + 'static,
    {
        Self {
            type_of: WidgetType::TextInput {
                value: value.into(),
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn select<F>(label: impl Into<String>, options: Vec<String>, onchange: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Select {
                label: label.into(),
                options,
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn text(text: impl Into<Value<String>>) -> Self {
        Self {
            type_of: WidgetType::Text { text: text.into() },
            children: None,
            style: Style::default(),
        }
    }

    pub fn icon(source: impl Into<String>) -> Self {
        Self {
            type_of: WidgetType::Icon {
                source: source.into(),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn image(source: impl Into<String>) -> Self {
        Self {
            type_of: WidgetType::Image {
                source: source.into(),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn progress(value: impl Into<Value<f64>>, min: f64, max: f64) -> Self {
        Self {
            type_of: WidgetType::ProgressBar {
                value: value.into(),
                min,
                max,
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn link<F>(label: impl Into<String>, onclick: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Hyperlink {
                label: label.into(),
                onclick: Box::new(onclick),
            },
            children: None,
            style: Style::default(),
        }
    }

    pub fn separator() -> Self {
        Self {
            type_of: WidgetType::Separator,
            children: None,
            style: Style::default(),
        }
    }

    pub fn canvas(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Canvas,
            children: Some(children),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}
