use crate::{
    DIRTY, Widget,
    error::{Error, Result},
    signal::Flags,
    text::Text,
    widgets::WidgetType,
};
use bounds::Bounds;
use taffy::{Size, TaffyTree};
use tessellate::Tessellate;

mod bounds;
mod tessellate;

pub struct Layout {
    pub tree: Widget,
    pub text: Text,
}

impl Layout {
    pub fn new(layout: Widget) -> Result<Self> {
        match layout.type_of {
            WidgetType::MainPanel => {
                let mut text = Text::new(2048);
                let tree = Self::init_buffer(layout, &mut text);
                Ok(Self { tree, text })
            }

            _ => Err(Error::InvalidRootWidget),
        }
    }

    pub fn flags() -> Flags {
        DIRTY.with(|f| f.get())
    }

    pub fn build(&mut self, width: f32, height: f32) {
        let mut tree = TaffyTree::new();
        let root = Bounds::build(&self.tree, &mut tree);

        tree.compute_layout(
            root,
            Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        )
        .unwrap();

        Tessellate::tessellate(&self.tree, &tree, root);

        DIRTY.with(|f| f.set(Flags::UNSIGNALED));
    }

    fn init_buffer(mut widget: Widget, text_context: &mut Text) -> Widget {
        if let Some(children) = widget.children {
            let new_children: Vec<_> = children
                .into_iter()
                .map(|mut child| match child.type_of {
                    WidgetType::Text { .. } | WidgetType::TextInput { .. } => {
                        let font_size = child.style.font_size.unwrap_or(14.0);
                        let line_height = child.style.line_height.unwrap_or(20.0);

                        child.buffer = Some(text_context.create_buffer(font_size, line_height));

                        child
                    }
                    _ => Self::init_buffer(child, text_context),
                })
                .collect();

            widget.children = Some(new_children);
        }

        widget
    }
}
