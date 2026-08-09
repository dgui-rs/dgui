use taffy::TaffyTree;

use crate::{DIRTY, Widget, signal::Flags};

mod tessellation;

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

    pub fn build(&mut self) {
        let _layout = Self::create_layout(&self.tree);

        DIRTY.with(|f| f.set(Flags::UNSIGNALED));
    }

    fn create_layout(_tree: &Widget) -> TaffyTree {
        let layout_tree = TaffyTree::new();

        layout_tree
    }
}
