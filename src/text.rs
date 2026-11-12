use std::collections::HashMap;

use cosmic_text::{Attrs, Buffer, CacheKey, FontSystem, Metrics, Shaping, SwashCache};
use etagere::{AtlasAllocator, size2};

use crate::text::atlas::Atlas;

pub mod atlas;
mod editor;

#[derive(Clone, Copy, Debug)]
pub struct CachedGlyph {
    pub rect: [u32; 4],
    pub offset: [i32; 2],
}

pub struct Text {
    pub fontsystem: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas: Atlas,
}

impl Text {
    pub fn new(atlas_size: i32) -> Self {
        Self {
            fontsystem: FontSystem::new(),
            swash_cache: SwashCache::new(),
            atlas: Atlas {
                allocator: AtlasAllocator::new(size2(atlas_size, atlas_size)),
                size: atlas_size,
                data: vec![0; (atlas_size * atlas_size * 4) as usize],
                cache: HashMap::new(),
            },
        }
    }

    pub fn create_buffer(&mut self, font_size: f32, line_height: f32) -> Buffer {
        Buffer::new(&mut self.fontsystem, Metrics::new(font_size, line_height))
    }

    pub fn set_text(&mut self, buffer: &mut Buffer, text: &str, attrs: Attrs) {
        buffer.set_text(text, &attrs, Shaping::Advanced, None);
    }

    pub fn measure(&mut self, buffer: &mut Buffer, available_width: Option<f32>) -> (f32, f32) {
        buffer.set_size(available_width, None);
        buffer.shape_until_scroll(&mut self.fontsystem, false);

        let mut max_width = 0.0_f32;
        let mut total_height = 0.0_f32;

        for run in buffer.layout_runs() {
            max_width = max_width.max(run.line_w);
            total_height += run.line_height;
        }

        (max_width, total_height)
    }

    pub fn find_glyph(&self, key: CacheKey) -> Option<&CachedGlyph> {
        self.atlas.cache.get(&key)
    }
}
