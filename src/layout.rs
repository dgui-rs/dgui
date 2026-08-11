use crate::{DIRTY, Widget, signal::Flags};
use bounds::Bounds;
use taffy::{Size, TaffyTree};
use tessellate::Tessellate;

mod bounds;
mod tessellate;

pub struct Layout {
    pub tree: Widget,
}

impl Layout {
    pub fn new(layout: Widget) -> Self {
        Self { tree: layout }
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
}
