use taffy::TaffyTree;

use crate::{
    Context, bounds,
    output::{LayoutBuffer, Mesh},
    widgets::{Widget, WidgetType},
};

pub struct Tessellate;

impl Tessellate {
    pub fn build(taffy_tree: &TaffyTree<bounds::NodeContext>, ctx: &mut Context) -> LayoutBuffer {
        let mut buffer: Option<LayoutBuffer> = None;

        if let Some(tree) = ctx.tree.as_ref() {
            let _text = &mut ctx.text;

            let _mesh = Self::tessellate(taffy_tree, tree);
        }

        buffer.unwrap()
    }

    fn tessellate(taffy: &TaffyTree<bounds::NodeContext>, tree: &Widget) -> Vec<Mesh> {
        let mut mesh: Vec<Mesh> = vec![];

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
                | WidgetType::Accordion => {
                    let m = Self::tessellate(taffy, tree);
                }

                WidgetType::Canvas => {}

                WidgetType::Text { .. } => {}

                WidgetType::Link { .. } => {}

                WidgetType::Button { .. } => {}

                WidgetType::Checkbox { .. } => {}

                WidgetType::DragValue { .. } => {}

                WidgetType::Image { .. } => {}

                WidgetType::Input { .. } => {}

                WidgetType::Separator => {}

                WidgetType::Slider { .. } => {}

                WidgetType::Svg { .. } => {}

                WidgetType::Switch { .. } => {}

                WidgetType::Node { .. } => {}

                WidgetType::Select { .. } => {}

                WidgetType::RadioButton { .. } => {}

                WidgetType::Progress { .. } => {}

                WidgetType::Lightness => {}

                WidgetType::Hue => {}

                WidgetType::Alpha => {}

                WidgetType::ColorPad => {}

                WidgetType::ColorWheel => {}
            }
        }

        mesh
    }

    fn rect() {}

    fn path() {}

    fn circle() {}

    fn raster_text() {}
}
