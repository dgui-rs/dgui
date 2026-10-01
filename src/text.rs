use std::collections::HashMap;

use cosmic_text::{
    Attrs, Buffer, CacheKey, FontSystem, Metrics, Shaping, SwashCache, SwashContent,
};
use etagere::{AllocId, AtlasAllocator, size2};

pub const ATLAS_PADDING: i32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct CachedGlyph {
    pub(crate) rect: [u32; 4],
    pub(crate) offset: [i32; 2],
    pub(crate) alloc_id: AllocId,
}

pub struct Text {
    pub(crate) font_system: FontSystem,
    pub(crate) swash_cache: SwashCache,
    pub(crate) atlas: AtlasAllocator,
    pub(crate) atlas_cache: HashMap<CacheKey, Option<CachedGlyph>>,
    pub(crate) atlas_size: u32,
    pub(crate) atlas_data: Vec<u8>,
}

impl Text {
    pub fn new(atlas_size: u32) -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            atlas: AtlasAllocator::new(size2(atlas_size as i32, atlas_size as i32)),
            atlas_cache: HashMap::new(),
            atlas_size,
            atlas_data: vec![0; (atlas_size * atlas_size * 4) as usize],
        }
    }

    pub fn create_buffer(&mut self, font_size: f32, line_height: f32) -> Buffer {
        Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height))
    }

    pub fn shape_and_measure(
        font_system: &mut FontSystem,
        buffer: &mut Buffer,
        text: &str,
        available_width: Option<f32>,
    ) -> (f32, f32) {
        buffer.set_size(available_width, None);
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);

        buffer.shape_until_scroll(font_system, false);

        let mut max_width = 0.0_f32;
        let mut lines = 0;

        for run in buffer.layout_runs() {
            max_width = max_width.max(run.line_w);
            lines += 1;
        }

        let line_height = buffer.metrics().line_height;
        let total_height = lines as f32 * line_height;

        (max_width, total_height)
    }

    pub fn atlas_allocation(&mut self, buffer: &Buffer) {
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical_glyph = glyph.physical((0.0, 0.0), 1.0);
                let key = physical_glyph.cache_key;

                if !self.atlas_cache.contains_key(&key) {
                    let Some(image) = self.swash_cache.get_image(&mut self.font_system, key) else {
                        self.atlas_cache.insert(key, None);
                        continue;
                    };

                    let width = image.placement.width as i32 + ATLAS_PADDING * 2;
                    let height = image.placement.height as i32 + ATLAS_PADDING * 2;

                    let alloc = match self.atlas.allocate(size2(width, height)) {
                        Some(alloc) => alloc,
                        None => {
                            self.atlas.clear();
                            self.atlas_cache.clear();
                            self.atlas_data.fill(0);
                            self.atlas
                                .allocate(size2(width, height))
                                .expect("Glyph is too large to fit in the atlas at all!")
                        }
                    };

                    let x0 = alloc.rectangle.min.x as u32 + ATLAS_PADDING as u32;
                    let y0 = alloc.rectangle.min.y as u32 + ATLAS_PADDING as u32;
                    let x1 = x0 + image.placement.width;
                    let y1 = y0 + image.placement.height;

                    let glyph_w = image.placement.width as usize;
                    let glyph_h = image.placement.height as usize;
                    let atlas_pitch = self.atlas_size as usize * 4;

                    match image.content {
                        SwashContent::Mask => {
                            for row in 0..glyph_h {
                                let dest_start =
                                    (y0 as usize + row) * atlas_pitch + (x0 as usize * 4);
                                let src_start = row * glyph_w;
                                for col in 0..glyph_w {
                                    let dest_pixel = dest_start + col * 4;
                                    self.atlas_data[dest_pixel] = 255;
                                    self.atlas_data[dest_pixel + 1] = 255;
                                    self.atlas_data[dest_pixel + 2] = 255;
                                    self.atlas_data[dest_pixel + 3] = image.data[src_start + col];
                                }
                            }
                        }
                        SwashContent::Color => {
                            for row in 0..glyph_h {
                                let dest_start =
                                    (y0 as usize + row) * atlas_pitch + (x0 as usize * 4);
                                let src_start = row * glyph_w * 4;
                                let len = glyph_w * 4;
                                self.atlas_data[dest_start..dest_start + len]
                                    .copy_from_slice(&image.data[src_start..src_start + len]);
                            }
                        }
                        _ => {
                            self.atlas_cache.insert(key, None);
                            continue;
                        }
                    }

                    self.atlas_cache.insert(
                        key,
                        Some(CachedGlyph {
                            rect: [x0, y0, x1, y1],
                            offset: [image.placement.left, image.placement.top],
                            alloc_id: alloc.id,
                        }),
                    );
                }
            }
        }
    }
}
