use taffy::TaffyTree;

use crate::{DIRTY, Result, Widget, signal::Flags};

mod tessellation;

pub struct Layout {
    pub tree: Widget,
}

impl Layout {
    pub fn new(layout: Widget) -> Result<Self> {
        Ok(Self { tree: layout })
    }

    pub fn flags() -> Result<Flags> {
        Ok(DIRTY.with(|f| f.get()))
    }

    pub fn build(&mut self) {
        let _layout = Self::create_layout(&self.tree);

        DIRTY.with(|f| f.set(Flags::UNSIGNALED));
    }

    fn create_layout(tree: &Widget) -> TaffyTree {
        let mut layout_tree = TaffyTree::new();

        layout_tree
    }
}
