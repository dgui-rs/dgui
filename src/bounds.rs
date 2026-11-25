use taffy::{AvailableSpace, NodeId, Size, TaffyTree};

use crate::{
    Context, WindowSize,
    text::Text,
    widgets::{Widget, WidgetType},
};

pub struct TextContext {
    layout: parley::Layout<[f32; 4]>,
}

pub(crate) enum NodeContext {
    Text(TextContext),
}

pub struct Bounds;

impl Bounds {
    pub fn compute(ctx: &mut Context, window_size: WindowSize) -> TaffyTree<NodeContext> {
        let mut tree: TaffyTree<NodeContext> = TaffyTree::new();

        let widget_tree = ctx.tree.as_ref().unwrap();

        let text = &mut ctx.text;

        let nodes = Self::build(text, &mut tree, widget_tree);

        let node = tree
            .new_with_children(widget_tree.styles.layout.clone(), &nodes)
            .unwrap();

        widget_tree.node_id.set(Some(node));

        tree.compute_layout_with_measure(
            node,
            Size {
                width: AvailableSpace::Definite(window_size.width),
                height: AvailableSpace::Definite(window_size.height),
            },
            |inputs, _node_id, node_context, style| {
                taffy::compute_leaf_layout(
                    inputs,
                    style,
                    |_, _| 0.0,
                    |known_dimensions, available_space| {
                        Self::measure(known_dimensions, available_space, node_context)
                    },
                )
            },
        )
        .unwrap();

        tree
    }

    fn build(text: &mut Text, taffy: &mut TaffyTree<NodeContext>, tree: &Widget) -> Vec<NodeId> {
        let mut nodes: Vec<NodeId> = vec![];

        for widget in &tree.children {
            match widget.type_of {
                WidgetType::Panel
                | WidgetType::Modal
                | WidgetType::Column
                | WidgetType::Row
                | WidgetType::Grid { .. }
                | WidgetType::Scrollarea
                | WidgetType::Clickable
                | WidgetType::Menu
                | WidgetType::Accordion
                | WidgetType::Canvas => {
                    let children = Self::build(text, taffy, widget);

                    let node = taffy
                        .new_with_children(widget.styles.layout.clone(), &children)
                        .unwrap();
                    nodes.push(node);

                    widget.node_id.set(Some(node));
                }

                WidgetType::Text { .. } | WidgetType::Button { .. } | WidgetType::Link { .. } => {
                    let layout = text.shape(widget, 1.0);

                    let context = NodeContext::Text(TextContext { layout });

                    let node = taffy
                        .new_leaf_with_context(widget.styles.layout.clone(), context)
                        .unwrap();
                    nodes.push(node);

                    widget.node_id.set(Some(node));
                }

                _ => {
                    let node = taffy.new_leaf(widget.styles.layout.clone()).unwrap();
                    nodes.push(node);

                    widget.node_id.set(Some(node));
                }
            }
        }

        nodes
    }

    fn measure(
        known_dimensions: taffy::geometry::Size<Option<f32>>,
        available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
        node_context: Option<&mut NodeContext>,
    ) -> taffy::Size<f32> {
        if let Size {
            width: Some(width),
            height: Some(height),
        } = known_dimensions
        {
            return Size { width, height };
        }

        match node_context {
            None => Size::ZERO,
            Some(NodeContext::Text(context)) => {
                let width = known_dimensions.width.unwrap_or_else(|| {
                    let widths = context.layout.calculate_content_widths();

                    match available_space.width {
                        AvailableSpace::MinContent => widths.min,

                        AvailableSpace::MaxContent => widths.max,

                        AvailableSpace::Definite(limit) => limit.min(widths.max).max(widths.min),
                    }
                    .ceil()
                });

                context.layout.break_all_lines(Some(width));

                let height = known_dimensions
                    .height
                    .unwrap_or_else(|| context.layout.height());

                Size { width, height }
            }
        }
    }
}
