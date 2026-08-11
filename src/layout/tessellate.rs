use taffy::{NodeId, TaffyTree};

use crate::Widget;

pub struct Tessellate;

impl Tessellate {
    pub fn tessellate(widget_tree: &Widget, taffy_tree: &TaffyTree<()>, node: NodeId) {}

    fn rect() {}

    fn circle() {}

    fn ring() {}

    fn path() {}

    fn quad_uv() {}
}
