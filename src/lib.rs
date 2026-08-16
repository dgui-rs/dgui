use std::{cell::Cell, rc::Rc};

use cosmic_text::Buffer;

use crate::{
    events::Event,
    signal::{Flags, Value},
    styles::Style,
    widgets::WidgetType,
};

pub mod error;
mod events;
pub mod layout;
pub mod signal;
pub mod styles;
mod text;
mod widgets;

thread_local! {
  pub(crate)  static DIRTY: Cell<Flags> = const { Cell::new(Flags::UNSIGNALED) };
}

pub struct Widget {
    pub(crate) type_of: WidgetType,
    pub(crate) children: Option<Vec<Widget>>,
    pub(crate) style: Style,
    pub(crate) buffer: Option<Buffer>,
}

impl Widget {
    pub fn main(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::MainPanel,
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn panel(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Panel,
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn scrollarea(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::ScrollArea,
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn tabs<F>(children: Vec<Widget>, active: impl Into<Value<u8>>, onchange: F) -> Self
    where
        F: Fn(u8) + 'static,
    {
        let onchange = Rc::new(onchange);

        let tabs: Vec<Widget> = children
            .into_iter()
            .filter(|widget| matches!(widget.type_of, WidgetType::Tab { .. }))
            .collect();

        let header: Vec<Widget> = tabs
            .iter()
            .enumerate()
            .filter_map(|(index, widget)| match &widget.type_of {
                WidgetType::Tab { label } => {
                    let onchange = Rc::clone(&onchange);
                    let index = index as u8;

                    Some(Widget::button(
                        vec![Widget::text(label.get())],
                        move || {
                            onchange(index);
                        },
                        || {},
                    ))
                }

                _ => None,
            })
            .collect();

        Self {
            type_of: WidgetType::Tabs {
                active: active.into(),
                onchange,
            },
            children: Some(vec![Widget::panel(header), Widget::panel(tabs)]),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn tab(children: Vec<Widget>, label: Value<String>) -> Self {
        Self {
            type_of: WidgetType::Tab { label },
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn collapsible<F>(
        label: String,
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
            },
            children: Some(vec![
                Widget::button(vec![Widget::text(label)], ontoggle, || {}),
                Widget::panel(children),
            ]),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn splitter(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Splitter,
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn window(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Window,
            children: Some(children),
            style: Style::default(),
            buffer: None,
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
            buffer: None,
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
            buffer: None,
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
            buffer: None,
        }
    }

    pub fn radio_button<F>(selected: impl Into<Value<bool>>, onchange: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::RadioButton {
                selected: selected.into(),
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
            buffer: None,
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
            buffer: None,
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
            buffer: None,
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
            buffer: None,
        }
    }

    pub fn select<F>(default: impl Into<Value<String>>, options: Vec<String>, onchange: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Select {
                default: default.into(),
                options,
                onchange: Box::new(onchange),
            },
            children: None,
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn text(text: impl Into<Value<String>>) -> Self {
        Self {
            type_of: WidgetType::Text { text: text.into() },
            children: None,
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn icon(source: impl Into<Value<String>>) -> Self {
        Self {
            type_of: WidgetType::Icon {
                source: source.into(),
            },
            children: None,
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn image(source: impl Into<Value<String>>) -> Self {
        Self {
            type_of: WidgetType::Image {
                source: source.into(),
            },
            children: None,
            style: Style::default(),
            buffer: None,
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
            buffer: None,
        }
    }

    pub fn link<F>(children: Vec<Widget>, onclick: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Hyperlink {
                onclick: Box::new(onclick),
            },
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn separator() -> Self {
        Self {
            type_of: WidgetType::Separator,
            children: None,
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn canvas(children: Vec<Widget>) -> Self {
        Self {
            type_of: WidgetType::Canvas,
            children: Some(children),
            style: Style::default(),
            buffer: None,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}
