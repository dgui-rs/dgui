#![allow(dead_code)]

use std::{cell::Cell, sync::Arc};

use parley::fontique::Blob;

use crate::{
    bounds::Bounds,
    output::LayoutBuffer,
    signal::{Flags, TextValue, Value},
    style::{Color, Style},
    tessellate::Tessellate,
    text::Text,
    widgets::{Widget, WidgetType},
};

mod bounds;
pub mod event;
pub mod output;
pub mod signal;
pub mod style;
mod tessellate;
mod text;
mod widgets;

thread_local! {
  pub  static DIRTY: Cell<Flags> = const { Cell::new(Flags::UNSIGNALED) };
}

pub struct WindowSize {
    width: f32,
    height: f32,
}

pub struct Context {
    pub tree: Option<Widget>,
    pub(crate) layout_buffer: Option<LayoutBuffer>,
    pub(crate) text: Text,
    pub(crate) dpi: f32,
}

impl Context {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tree: None,
            layout_buffer: None,
            text: Text::new(),
            dpi: 1.0,
        }
    }

    pub fn builder(mut self, tree: Widget) -> Self {
        self.tree = Some(tree);
        self
    }

    pub fn set_dpi(mut self, dpi: f32) -> Self {
        self.dpi = dpi;
        self
    }

    pub fn compute(&mut self, window_size: WindowSize) -> LayoutBuffer {
        if self.layout_buffer.is_none() {
            DIRTY.with(|flags| flags.set(Flags::SIGNALED));
        }

        let flags = DIRTY.with(|f| f.get());

        match flags {
            Flags::SIGNALED => {
                let computed_layout = Bounds::compute(self, window_size);

                let geometry = Tessellate::build(&computed_layout, self);

                self.layout_buffer = Some(geometry);

                DIRTY.with(|flags| flags.set(Flags::UNSIGNALED));

                geometry
            }

            Flags::UNSIGNALED => self.layout_buffer.unwrap(),
        }
    }

    //display

    pub fn text(&self, text: impl TextValue) -> Widget {
        Widget {
            type_of: WidgetType::Text {
                text: text.into_boxed(),
                font_name: None,
            },
            children: vec![],
            styles: Style::default().background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn typography(&mut self, text: impl TextValue, path: &str) -> Widget {
        let font_data = std::fs::read(path).unwrap();

        let blob = Blob::new(Arc::new(font_data));

        let registered = self.text.font.collection.register_fonts(blob, None);

        let font = registered.first().map(|(family, _)| {
            self.text
                .font
                .collection
                .family_name(*family)
                .unwrap()
                .to_owned()
        });

        Widget {
            type_of: WidgetType::Text {
                text: text.into_boxed(),
                font_name: font,
            },
            children: vec![],
            styles: Style::default().background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn link(&self, label: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Link {
                label: label.into(),
            },
            children: vec![],
            styles: Style::default()
                .background_color(Color::TRANSPARENT)
                .underline(),
            node_id: Cell::new(None),
        }
    }

    pub fn image(&self, src: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Image { src: src.into() },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn progress(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: WidgetType::Progress {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn separator(&self) -> Widget {
        Widget {
            type_of: WidgetType::Separator,
            children: vec![],
            styles: Style::default().height(style::Dimension::Px(1.0)),
            node_id: Cell::new(None),
        }
    }

    pub fn svg(&self, src: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Svg { src: src.into() },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    //layout

    pub fn panel(&self) -> Widget {
        Widget {
            type_of: WidgetType::Panel,
            children: vec![],
            styles: Style::default()
                .width(style::Dimension::Percent(100.0))
                .height(style::Dimension::Percent(100.0)),
            node_id: Cell::new(None),
        }
    }

    pub fn modal(&self) -> Widget {
        Widget {
            type_of: WidgetType::Modal,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn column(&self) -> Widget {
        Widget {
            type_of: WidgetType::Column,
            children: vec![],
            styles: Style::default()
                .flex_direction(style::FlexDirection::Column)
                .background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn row(&self) -> Widget {
        Widget {
            type_of: WidgetType::Row,
            children: vec![],
            styles: Style::default()
                .flex_direction(style::FlexDirection::Row)
                .background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn grid(&self, cols: u8, rows: u8) -> Widget {
        Widget {
            type_of: WidgetType::Grid { cols, rows },
            children: vec![],
            styles: Style::default()
                .display(style::Display::Grid)
                .background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn scrollarea(&self) -> Widget {
        Widget {
            type_of: WidgetType::Scrollarea,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    //interactive

    pub fn button(&self, text: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Button { text: text.into() },
            children: vec![],
            styles: Style::default().background_color(Color::NEUTRAL_800),
            node_id: Cell::new(None),
        }
    }

    pub fn clickable(&self) -> Widget {
        Widget {
            type_of: WidgetType::Clickable,
            children: vec![],
            styles: Style::default().background_color(Color::TRANSPARENT),
            node_id: Cell::new(None),
        }
    }

    pub fn checkbox(&self, checked: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: WidgetType::Checkbox {
                checked: checked.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn radio_button(&self, checked: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: WidgetType::RadioButton {
                checked: checked.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn select(&self, selected: impl Into<Value<String>>, options: Vec<String>) -> Widget {
        Widget {
            type_of: WidgetType::Select {
                selected: selected.into(),
                options,
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn slider(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: WidgetType::Slider {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn drag_value(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: WidgetType::DragValue {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn input(&mut self, value: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Input {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn switch(&self, toggle: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: WidgetType::Switch {
                toggle: toggle.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    //special

    pub fn menu(&self) -> Widget {
        Widget {
            type_of: WidgetType::Menu,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn accordion(&self) -> Widget {
        Widget {
            type_of: WidgetType::Accordion,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    // Graphs and custom rendering

    pub fn canvas(&self) -> Widget {
        Widget {
            type_of: WidgetType::Canvas,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn node(&self, title: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: WidgetType::Node {
                title: title.into(),
            },
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    //color
    pub fn color_wheel(&self) -> Widget {
        Widget {
            type_of: WidgetType::ColorWheel,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn hue_strip(&self) -> Widget {
        Widget {
            type_of: WidgetType::Hue,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn lightness_strip(&self) -> Widget {
        Widget {
            type_of: WidgetType::Lightness,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn alpha_strip(&self) -> Widget {
        Widget {
            type_of: WidgetType::Alpha,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }

    pub fn color_pad(&self) -> Widget {
        Widget {
            type_of: WidgetType::ColorPad,
            children: vec![],
            styles: Style::default(),
            node_id: Cell::new(None),
        }
    }
}
