use taffy::{NodeId, TaffyTree};

use crate::{Widget, layout::TextContext};

pub struct Tessellate;

impl Tessellate {
    pub fn tessellate(_widget_tree: &Widget, _taffy_tree: &TaffyTree<TextContext>, _node: NodeId) {}

    fn rect() {}

    fn circle() {}

    fn ring() {}

    fn path() {}

    fn quad_uv() {}
}
