use lyon::{
    math::{Box2D, Point, point},
    path::{Path, Winding, builder::BorderRadii},
    tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
        StrokeVertex, VertexBuffers,
    },
};
use taffy::TaffyTree;

use crate::{
    Widget,
    draw::{Mesh, Texture, Vertex},
    layout::TextContext,
    styles::Color,
    text::Text,
    widgets::WidgetType,
};

pub struct Tessellate;

impl Tessellate {
    pub fn tessellate(
        tree: &mut Widget,
        taffy_tree: &mut TaffyTree<TextContext>,
        text: &mut Text,
    ) -> Vec<Mesh> {
        let mut meshes: Vec<Mesh> = Vec::new();
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            scissor: None,
            texture: Texture::Atlas,
        };

        let mut stack = vec![(tree, 0.0_f32, 0.0_f32, None::<[u32; 4]>)];

        while let Some((widget, parent_x, parent_y, current_scissor)) = stack.pop() {
            let Some(id) = widget.id else {
                continue;
            };

            let layout = taffy_tree.layout(id).unwrap();
            let x = parent_x + layout.location.x;
            let y = parent_y + layout.location.y;
            let w = layout.size.width;
            let h = layout.size.height;

            let active_scissor = if matches!(widget.type_of, WidgetType::ScrollArea) {
                Some([
                    x.max(0.0) as u32,
                    y.max(0.0) as u32,
                    w.max(0.0) as u32,
                    h.max(0.0) as u32,
                ])
            } else {
                current_scissor
            };

            Self::clip(&mut mesh, &mut meshes, active_scissor);

            if let Some(bg) = widget.style.background_color {
                let radius = widget.style.border_radius.unwrap_or(0.0);
                Self::rect(&mut mesh, x, y, w, h, radius, bg.to_rgba8());
            }

            match &widget.type_of {
                WidgetType::Text { .. } => {
                    if let Some(ctx) = taffy_tree.get_node_context_mut(id)
                        && let Some(buffer) = ctx.buffer.take()
                    {
                        text.atlas_allocation(&buffer);

                        let color = widget.style.color.unwrap_or(Color::WHITE).to_rgba8();
                        let atlas_size = text.atlas_size as f32;

                        for run in buffer.layout_runs() {
                            for glyph in run.glyphs {
                                let phys = glyph.physical((x, y + run.line_y), 1.0);
                                if let Some(Some(cached)) = text.atlas_cache.get(&phys.cache_key) {
                                    let gx = (phys.x + cached.offset[0]) as f32;
                                    let gy = (phys.y - cached.offset[1]) as f32;
                                    let gw = (cached.rect[2] - cached.rect[0]) as f32;
                                    let gh = (cached.rect[3] - cached.rect[1]) as f32;

                                    let u0 = cached.rect[0] as f32 / atlas_size;
                                    let v0 = cached.rect[1] as f32 / atlas_size;
                                    let u1 = cached.rect[2] as f32 / atlas_size;
                                    let v1 = cached.rect[3] as f32 / atlas_size;

                                    Self::quad_uv(
                                        &mut mesh,
                                        gx,
                                        gy,
                                        gw,
                                        gh,
                                        [u0, v0],
                                        [u1, v1],
                                        color,
                                    );
                                }
                            }
                        }

                        widget.buffer = Some(buffer);
                    }
                }

                WidgetType::RadioButton { selected, .. } => {
                    let r = w.min(h) * 0.5;
                    let cx = x + r;
                    let cy = y + r;
                    let color = widget.style.color.unwrap_or(Color::WHITE).to_rgba8();

                    Self::ring(&mut mesh, cx, cy, (r - 1.0).max(0.0), 2.0, color);
                    if selected.get() {
                        Self::circle(&mut mesh, cx, cy, r * 0.5, color);
                    }
                }

                WidgetType::Checkbox { checked, .. } => {
                    let radius = widget.style.border_radius.unwrap_or(4.0);
                    let color = widget.style.color.unwrap_or(Color::BLUE_500).to_rgba8();

                    if checked.get() {
                        Self::rect(&mut mesh, x, y, w, h, radius, color);
                    }
                }

                WidgetType::Switch { checked, .. } => {
                    let r = h * 0.5;
                    let track_color = if checked.get() {
                        widget.style.color.unwrap_or(Color::BLUE_500).to_rgba8()
                    } else {
                        Color::NEUTRAL_700.to_rgba8()
                    };
                    Self::rect(&mut mesh, x, y, w, h, r, track_color);

                    let thumb_r = (r - 2.0).max(0.0);
                    let thumb_cx = if checked.get() { x + w - r } else { x + r };
                    Self::circle(&mut mesh, thumb_cx, y + r, thumb_r, Color::WHITE.to_rgba8());
                }

                WidgetType::ProgressBar { value, min, max } => {
                    let radius = widget.style.border_radius.unwrap_or(4.0);
                    let range = (max - min).max(f64::EPSILON);
                    let pct = ((value.get() - min) / range).clamp(0.0, 1.0) as f32;
                    let fill_color = widget.style.color.unwrap_or(Color::BLUE_500).to_rgba8();

                    Self::rect(&mut mesh, x, y, w * pct, h, radius, fill_color);
                }

                WidgetType::Slider {
                    value, min, max, ..
                } => {
                    let range = (max - min).max(f64::EPSILON);
                    let pct = ((value.get() - min) / range).clamp(0.0, 1.0) as f32;

                    let track_h = 4.0;
                    let track_y = y + (h - track_h) * 0.5;
                    let active_color = widget.style.color.unwrap_or(Color::BLUE_500).to_rgba8();

                    Self::rect(
                        &mut mesh,
                        x,
                        track_y,
                        w,
                        track_h,
                        2.0,
                        Color::NEUTRAL_700.to_rgba8(),
                    );
                    Self::rect(&mut mesh, x, track_y, w * pct, track_h, 2.0, active_color);

                    let thumb_r = (h * 0.4).max(6.0);
                    Self::circle(
                        &mut mesh,
                        x + w * pct,
                        y + h * 0.5,
                        thumb_r,
                        Color::WHITE.to_rgba8(),
                    );
                }

                WidgetType::Separator => {
                    let color = widget.style.color.unwrap_or(Color::NEUTRAL_700).to_rgba8();
                    Self::rect(&mut mesh, x, y, w, h.max(1.0), 0.0, color);
                }

                _ => {}
            }

            if let Some(children) = widget.children.as_mut() {
                for child in children.iter_mut().rev() {
                    stack.push((child, x, y, active_scissor));
                }
            }
        }

        if !mesh.vertices.is_empty() {
            meshes.push(mesh);
        }

        meshes
    }

    fn clip(mesh: &mut Mesh, meshes: &mut Vec<Mesh>, target_scissor: Option<[u32; 4]>) {
        if mesh.scissor != target_scissor {
            if !mesh.vertices.is_empty() {
                let finished = std::mem::replace(
                    mesh,
                    Mesh {
                        vertices: Vec::new(),
                        indices: Vec::new(),
                        scissor: target_scissor,
                        texture: Texture::Atlas,
                    },
                );
                meshes.push(finished);
            } else {
                mesh.scissor = target_scissor;
            }
        }
    }

    fn rect(mesh: &mut Mesh, x: f32, y: f32, w: f32, h: f32, radius: f32, color: [u8; 4]) {
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        if radius <= 0.0 {
            Self::quad_uv(mesh, x, y, w, h, [-1.0, -1.0], [-1.0, -1.0], color);
            return;
        }

        let mut builder = Path::builder();
        let box2d = Box2D::new(point(x, y), point(x + w, y + h));
        builder.add_rounded_rectangle(&box2d, &BorderRadii::new(radius), Winding::Positive);
        let path = builder.build();

        Self::path(mesh, &path, true, 0.0, color);
    }

    fn circle(mesh: &mut Mesh, cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
        if radius <= 0.0 {
            return;
        }

        let mut builder = Path::builder();
        builder.add_circle(Point::new(cx, cy), radius, Winding::Positive);
        let path = builder.build();

        Self::path(mesh, &path, true, 0.0, color);
    }

    fn ring(mesh: &mut Mesh, cx: f32, cy: f32, radius: f32, thickness: f32, color: [u8; 4]) {
        if radius <= 0.0 || thickness <= 0.0 {
            return;
        }

        let mut builder = Path::builder();
        builder.add_circle(Point::new(cx, cy), radius, Winding::Positive);
        let path = builder.build();

        Self::path(mesh, &path, false, thickness, color);
    }

    fn path(mesh: &mut Mesh, path: &Path, fill: bool, stroke_width: f32, color: [u8; 4]) {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        if fill {
            let mut tessellator = FillTessellator::new();
            let _ = tessellator.tessellate_path(
                path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vertex {
                    pos: vertex.position().to_array(),
                    uv: [-1.0, -1.0],
                    color,
                }),
            );
        } else {
            let mut tessellator = StrokeTessellator::new();
            let _ = tessellator.tessellate_path(
                path,
                &StrokeOptions::default().with_line_width(stroke_width),
                &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| Vertex {
                    pos: vertex.position().to_array(),
                    uv: [-1.0, -1.0],
                    color,
                }),
            );
        }

        let base = mesh.vertices.len() as u32;
        mesh.vertices.extend(geometry.vertices);
        mesh.indices
            .extend(geometry.indices.into_iter().map(|i| base + i));
    }

    #[allow(clippy::too_many_arguments)]
    fn quad_uv(
        mesh: &mut Mesh,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        uv_min: [f32; 2],
        uv_max: [f32; 2],
        color: [u8; 4],
    ) {
        let base = mesh.vertices.len() as u32;

        mesh.vertices.extend_from_slice(&[
            Vertex {
                pos: [x, y],
                uv: [uv_min[0], uv_min[1]],
                color,
            },
            Vertex {
                pos: [x + w, y],
                uv: [uv_max[0], uv_min[1]],
                color,
            },
            Vertex {
                pos: [x + w, y + h],
                uv: [uv_max[0], uv_max[1]],
                color,
            },
            Vertex {
                pos: [x, y + h],
                uv: [uv_min[0], uv_max[1]],
                color,
            },
        ]);

        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}
