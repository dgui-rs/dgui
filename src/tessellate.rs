use taffy::TaffyTree;

use crate::{Context, output::LayoutBuffer};

pub struct Tessellate;

impl Tessellate {
    pub fn build(_taffy_tree: &TaffyTree<()>, _ctx: &Context) -> LayoutBuffer {
        LayoutBuffer {}
    }
}
