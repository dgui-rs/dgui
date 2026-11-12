#![allow(dead_code)]

use std::cell::Cell;

use crate::{
    bounds::Bounds,
    output::LayoutBuffer,
    signal::{Flags, Value},
    style::Style,
    tessellate::Tessellate,
    text::Text,
    widgets::Widget,
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

pub struct Context {
    pub tree: Option<Widget>,
    pub(crate) layout_buffer: Option<LayoutBuffer>,
    pub(crate) text: Text,
}

impl Context {
    pub fn new(atlas_size: i32) -> Self {
        Self {
            tree: None,
            layout_buffer: None,
            text: Text::new(atlas_size),
        }
    }

    pub fn builder(mut self, tree: Widget) -> Self {
        self.tree = Some(tree);
        self
    }

    pub fn compute(&mut self) -> LayoutBuffer {
        if self.layout_buffer.is_none() {
            DIRTY.with(|flags| flags.set(Flags::SIGNALED));
        }

        let flags = DIRTY.with(|f| f.get());

        match flags {
            Flags::SIGNALED => {
                let computed_layout = Bounds::compute(self);

                let geometry = Tessellate::build(&computed_layout, self);

                self.layout_buffer = Some(geometry);

                DIRTY.with(|flags| flags.set(Flags::UNSIGNALED));

                geometry
            }

            Flags::UNSIGNALED => self.layout_buffer.unwrap(),
        }
    }

    //display

    pub fn text(&mut self, text: impl Into<Value<String>>) -> Widget {
        let buffer = self.text.create_buffer(15.0, 20.0);

        Widget {
            type_of: widgets::Type::Text { text: text.into() },
            children: vec![],
            styles: Style::default(),
            buffer: Some(buffer),
        }
    }

    pub fn link(&self, src: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: widgets::Type::Link { src: src.into() },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn image(&self, src: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: widgets::Type::Image { src: src.into() },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn progress(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: widgets::Type::Progress {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn separator(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Separator,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn svg(&self, src: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: widgets::Type::Svg { src: src.into() },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    //layout

    pub fn panel(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Panel,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn modal(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Modal,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn column(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Column,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn row(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Row,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn grid(&self, cols: u8, rows: u8) -> Widget {
        Widget {
            type_of: widgets::Type::Grid { cols, rows },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn scrollarea(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Scrollarea,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    //interactive

    pub fn button(&self, text: impl Into<Value<String>>) -> Widget {
        Widget {
            type_of: widgets::Type::Button { text: text.into() },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn image_button(&self) -> Widget {
        Widget {
            type_of: widgets::Type::ImageButton,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn checkbox(&self, checked: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: widgets::Type::Checkbox {
                checked: checked.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn radio_button(&self, checked: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: widgets::Type::RadioButton {
                checked: checked.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn select(&self, selected: impl Into<Value<String>>, options: Vec<String>) -> Widget {
        Widget {
            type_of: widgets::Type::Select {
                selected: selected.into(),
                options,
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn slider(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: widgets::Type::Slider {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn drag_value(&self, value: impl Into<Value<f32>>) -> Widget {
        Widget {
            type_of: widgets::Type::DragValue {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn input(&mut self, value: impl Into<Value<String>>) -> Widget {
        let buffer = self.text.create_buffer(15.0, 20.0);

        Widget {
            type_of: widgets::Type::Input {
                value: value.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: Some(buffer),
        }
    }

    pub fn switch(&self, toggle: impl Into<Value<bool>>) -> Widget {
        Widget {
            type_of: widgets::Type::Switch {
                toggle: toggle.into(),
            },
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    //special

    pub fn menu(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Menu,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn accordion(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Accordion,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    // Graphs and custom rendering

    pub fn canvas(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Canvas,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn node(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Node,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    //color
    pub fn color_wheel(&self) -> Widget {
        Widget {
            type_of: widgets::Type::ColorWheel,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn hue_strip(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Hue,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn lightness_strip(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Lightness,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn alpha_strip(&self) -> Widget {
        Widget {
            type_of: widgets::Type::Alpha,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }

    pub fn color_pad(&self) -> Widget {
        Widget {
            type_of: widgets::Type::ColorPad,
            children: vec![],
            styles: Style::default(),
            buffer: None,
        }
    }
}
