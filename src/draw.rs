use crate::text::atlas::DirtyRect;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct PaintBatch {
    pub mesh: Mesh,
    pub scissor: Option<[u32; 4]>,
}

pub struct Output<'a> {
    pub batches: Vec<PaintBatch>,
    pub atlas_pixels: &'a [u8],
    pub atlas_dirty: Option<DirtyRect>,
}
