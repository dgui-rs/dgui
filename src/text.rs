use cosmic_text::{Buffer, FontSystem, Metrics, SwashCache};
use etagere::{AtlasAllocator, Size};

pub struct Text {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas: AtlasAllocator,
    pub atlas_size: i32,
}

impl Text {
    pub fn new(atlas_size: i32) -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            atlas: AtlasAllocator::new(Size::new(atlas_size, atlas_size)),
            atlas_size,
        }
    }

    pub fn create_buffer(&mut self, font_size: f32, line_height: f32) -> Buffer {
        Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height))
    }

    pub fn render() {}
}
