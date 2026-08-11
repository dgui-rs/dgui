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
                        WidgetType::Tabs { active, header, .. } => match child.children.as_ref() {
                            Some(children) if !children.is_empty() => {
                                let header_buttons: Vec<_> = header
                                    .iter()
                                    .map(|h| Self::build_taffy_tree(h, taffy))
                                    .collect();

                                let header = taffy
                                    .new_with_children(taffy::Style::default(), &header_buttons)
                                    .unwrap();

                                let index = active.get() as usize;

                                let active_index = if index < children.len() { index } else { 0 };

                                let active_node =
                                    Self::build_taffy_tree(&children[active_index], taffy);

                                taffy
                                    .new_with_children(
                                        child.style.layout.clone(),
                                        &[header, active_node],
                                    )
                                    .unwrap()
                            }

                            _ => taffy.new_leaf(child.style.layout.clone()).unwrap(),
                        },
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

    fn tessellate(tree: &TaffyTree<()>, node: taffy::NodeId) {}
}
