use taffy::TaffyTree;

use crate::{Widget, widgets::WidgetType};

pub struct Bounds;

impl Bounds {
    pub fn build(widget: &Widget, tree: &mut TaffyTree<()>) -> taffy::NodeId {
        match &widget.children {
            Some(children) => {
                let nodes: Vec<_> = children
                    .iter()
                    .map(|child| match &child.type_of {
                        WidgetType::Tabs { active, .. } => {
                            let nodes = child.children.as_ref().unwrap();
                            let header = &nodes[0];
                            let content = &nodes[1];

                            if let Some(tabs) =
                                content.children.as_deref().filter(|t| !t.is_empty())
                            {
                                let header_node = Self::build(header, tree);

                                let index = active.get() as usize;
                                let active_index = if index < tabs.len() { index } else { 0 };

                                let active_node = Self::build(&tabs[active_index], tree);

                                tree.new_with_children(
                                    child.style.layout.clone(),
                                    &[header_node, active_node],
                                )
                                .unwrap()
                            } else {
                                tree.new_leaf(child.style.layout.clone()).unwrap()
                            }
                        }

                        WidgetType::Collapsible { expand, .. } => {
                            if let Some([header, content]) = child.children.as_deref() {
                                let header_node = Self::build(header, tree);

                                if expand.get() {
                                    let content_node = Self::build(content, tree);

                                    tree.new_with_children(
                                        child.style.layout.clone(),
                                        &[header_node, content_node],
                                    )
                                    .unwrap()
                                } else {
                                    tree.new_with_children(
                                        child.style.layout.clone(),
                                        &[header_node],
                                    )
                                    .unwrap()
                                }
                            } else {
                                tree.new_leaf(child.style.layout.clone()).unwrap()
                            }
                        }

                        _ => Self::build(child, tree),
                    })
                    .collect();

                tree.new_with_children(widget.style.layout.clone(), &nodes)
                    .unwrap()
            }

            None => tree.new_leaf(widget.style.layout.clone()).unwrap(),
        }
    }
}
