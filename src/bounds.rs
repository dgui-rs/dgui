use taffy::TaffyTree;

use crate::Context;

pub struct Bounds;

impl Bounds {
    pub fn compute(ctx: &Context) -> TaffyTree<()> {
        let tree = TaffyTree::new();

        let _text = &ctx.text;

        tree
    }
}
