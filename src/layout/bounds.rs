use taffy::TaffyTree;

use crate::{Widget, layout::TextContext, widgets::WidgetType};

pub struct Bounds;

impl Bounds {
    pub fn build(widget: &mut Widget, tree: &mut TaffyTree<TextContext>) -> taffy::NodeId {
        match &mut widget.children {
            Some(children) => {
                let nodes: Vec<_> = children
                    .iter_mut() // 1. Must be iter_mut()
                    .map(|child| match &child.type_of {
                        WidgetType::Tabs { active, .. } => {
                            let nodes = child.children.as_mut().unwrap();

                            let (header_slice, content_slice) = nodes.split_at_mut(1);
                            let header = &mut header_slice[0];
                            let content = &mut content_slice[0];

                            if let Some(tabs) = content.children.as_mut().filter(|t| !t.is_empty())
                            {
                                let header_node = Self::build(header, tree);

                                let index = active.get() as usize;
                                let active_index = if index < tabs.len() { index } else { 0 };

                                let active_node = Self::build(&mut tabs[active_index], tree);

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
                            if let Some(nodes) = child.children.as_mut() {
                                let (header_slice, content_slice) = nodes.split_at_mut(1);
                                let header = &mut header_slice[0];
                                let content = &mut content_slice[0];

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

            None => match &widget.type_of {
                WidgetType::Text { text } => {
                    let buffer = widget.buffer.take().expect("Buffer not initialized");

                    let text_context = crate::layout::TextContext {
                        buffer,
                        text: text.clone(),
                    };

                    tree.new_leaf_with_context(widget.style.layout.clone(), text_context)
                        .unwrap()
                }

                _ => tree.new_leaf(widget.style.layout.clone()).unwrap(),
            },
        }
    }
}
