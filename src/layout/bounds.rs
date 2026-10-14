use taffy::TaffyTree;

use crate::{Widget, layout::TextContext, widgets::WidgetType};

pub struct Bounds;

impl Bounds {
    pub fn build(widget: &mut Widget, tree: &mut TaffyTree<TextContext>) -> taffy::NodeId {
        widget.id = None;

        let id = match &widget.type_of {
            WidgetType::Tabs { active, .. } => {
                if let Some(nodes) = widget.children.as_mut() {
                    let (header_slice, content_slice) = nodes.split_at_mut(1);
                    let header = &mut header_slice[0];
                    let content = &mut content_slice[0];

                    let header_node = Self::build(header, tree);

                    let content_node =
                        if let Some(tabs) = content.children.as_mut().filter(|t| !t.is_empty()) {
                            let index = active.get() as usize;
                            let active_index = if index < tabs.len() { index } else { 0 };

                            let active_tab_node = Self::build(&mut tabs[active_index], tree);
                            tree.new_with_children(content.style.layout.clone(), &[active_tab_node])
                                .unwrap()
                        } else {
                            tree.new_leaf(content.style.layout.clone()).unwrap()
                        };

                    content.id = Some(content_node);

                    tree.new_with_children(
                        widget.style.layout.clone(),
                        &[header_node, content_node],
                    )
                    .unwrap()
                } else {
                    tree.new_leaf(widget.style.layout.clone()).unwrap()
                }
            }

            WidgetType::Collapsible { expand, .. } => {
                if let Some(nodes) = widget.children.as_mut() {
                    let (header_slice, content_slice) = nodes.split_at_mut(1);
                    let header = &mut header_slice[0];
                    let content = &mut content_slice[0];

                    let header_node = Self::build(header, tree);

                    if expand.get() {
                        let content_node = Self::build(content, tree);

                        tree.new_with_children(
                            widget.style.layout.clone(),
                            &[header_node, content_node],
                        )
                        .unwrap()
                    } else {
                        tree.new_with_children(widget.style.layout.clone(), &[header_node])
                            .unwrap()
                    }
                } else {
                    tree.new_leaf(widget.style.layout.clone()).unwrap()
                }
            }

            WidgetType::Text { text } => {
                let buffer = widget.buffer.take().expect("Buffer not initialized");

                let text_context = TextContext {
                    buffer: Some(buffer),
                    text: text.clone(),
                };

                tree.new_leaf_with_context(widget.style.layout.clone(), text_context)
                    .unwrap()
            }

            _ => match &mut widget.children {
                Some(children) => {
                    let nodes: Vec<_> = children
                        .iter_mut()
                        .map(|child| Self::build(child, tree))
                        .collect();

                    tree.new_with_children(widget.style.layout.clone(), &nodes)
                        .unwrap()
                }
                None => tree.new_leaf(widget.style.layout.clone()).unwrap(),
            },
        };

        widget.id = Some(id);
        id
    }
}
