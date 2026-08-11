#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub u64);

pub struct DrawBuffer {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub command: Vec<DrawCommand>,
}

pub struct DrawCommand {
    pub index_start: u32,
    pub index_count: u32,
    pub texture: Option<TextureId>,
}

pub struct Vertex {
    pub vertices: Vec<f32>,
    pub color: Vec<f32>,
}
