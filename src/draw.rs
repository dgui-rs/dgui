#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [u8; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Texture {
    Atlas,
    Custom(u64),
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub scissor: Option<[u32; 4]>,
    pub texture: Texture,
}

pub struct Output<'a> {
    pub list: Vec<Mesh>,
    pub atlas_pixels: &'a [u8],
    pub atlas_size: u32,
}
