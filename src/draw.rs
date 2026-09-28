#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [u8; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureSource {
    Atlas,
    Custom(u64),
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct DrawBatch {
    pub mesh: Mesh,
    pub scissor: Option<[u32; 4]>,
    pub texture: TextureSource,
}

pub struct Output<'a> {
    pub batches: Vec<DrawBatch>,
    pub atlas_pixels: &'a [u8],
}
