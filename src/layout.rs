use taffy::{Size, TaffyTree};

use crate::{DIRTY, Widget, signal::Flags, widgets::WidgetType};

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

        let root = Self::build_taffy_tree(&self.tree, &mut tree);

        tree.compute_layout(
            root,
            Size {
                width: taffy::AvailableSpace::Definite(width),
                height: taffy::AvailableSpace::Definite(height),
            },
        )
        .unwrap();

        Self::tessellate(&tree, root);

        DIRTY.with(|f| f.set(Flags::UNSIGNALED));
    }

    fn build_taffy_tree(widget: &Widget, taffy: &mut TaffyTree<()>) -> taffy::NodeId {
        match &widget.children {
            Some(children) => {
                let nodes: Vec<_> = children
                    .iter()
                    .map(|child| match &child.type_of {
                        WidgetType::Tabs { active, .. } => {
                            let nodes = child.children.as_ref().unwrap();
                            let header = &nodes[0];
                            let content = &nodes[1];

                            let tabs = content.children.as_ref().unwrap();

                            if tabs.is_empty() {
                                taffy.new_leaf(child.style.layout.clone()).unwrap()
                            } else {
                                let header_node = Self::build_taffy_tree(header, taffy);

                                let index = active.get() as usize;
                                let active_index = if index < tabs.len() { index } else { 0 };

                                let active_node =
                                    Self::build_taffy_tree(&tabs[active_index], taffy);

                                taffy
                                    .new_with_children(
                                        child.style.layout.clone(),
                                        &[header_node, active_node],
                                    )
                                    .unwrap()
                            }
                        }

                        WidgetType::Collapsible { expand, .. } => {
                            let header = Self::build_taffy_tree(&children[0], taffy);

                            if expand.get() {
                                let content = Self::build_taffy_tree(&children[1], taffy);

                                taffy
                                    .new_with_children(
                                        child.style.layout.clone(),
                                        &[header, content],
                                    )
                                    .unwrap()
                            } else {
                                taffy
                                    .new_with_children(child.style.layout.clone(), &[header])
                                    .unwrap()
                            }
                        }

                        _ => Self::build_taffy_tree(child, taffy),
                    })
                    .collect();

                taffy
                    .new_with_children(widget.style.layout.clone(), &nodes)
                    .unwrap()
            }

            None => taffy.new_leaf(widget.style.layout.clone()).unwrap(),
        }
    }

    fn tessellate(_tree: &TaffyTree<()>, _node: taffy::NodeId) {}
}
