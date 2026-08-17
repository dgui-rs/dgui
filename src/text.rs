pub mod atlas;
// pub mod input; // For future text input

use cosmic_text::{
    Attrs, Buffer, CacheKey, FontSystem, Metrics, Shaping, SwashCache, SwashContent,
};
use std::collections::HashMap;

use crate::{Widget, widgets::WidgetType};
pub use atlas::Atlas;

#[derive(Clone, Copy, Debug)]
pub struct CachedGlyph {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

pub struct Text {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub atlas: Atlas,
    pub cache: HashMap<CacheKey, Option<CachedGlyph>>,
}

impl Text {
    pub fn new(atlas_size: u32) -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            atlas: Atlas::new(atlas_size),
            cache: HashMap::new(),
        }
    }

    pub fn create_buffer(&mut self, font_size: f32, line_height: f32) -> Buffer {
        assert!(font_size > 0.0, "font_size must be > 0");
        assert!(line_height > 0.0, "line_height must be > 0");
        Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height))
    }

    pub fn render(&mut self, widget: &mut Widget) -> (f32, f32) {
        match &widget.type_of {
            WidgetType::Text { text } => {
                let Some(buffer) = widget.buffer.as_mut() else {
                    return (0.0, 0.0);
                };

                buffer.set_text(&text.get(), &Attrs::new(), Shaping::Advanced, None);
                buffer.shape_until_scroll(&mut self.font_system, false);

                let mut max_width = 0.0f32;
                let mut total_height = 0.0f32;
                let mut glyph_keys = Vec::new();

                for run in buffer.layout_runs() {
                    max_width = max_width.max(run.line_w);
                    total_height += run.line_height;

                    for glyph in run.glyphs {
                        let physical = glyph.physical((0.0, 0.0), 1.0);
                        glyph_keys.push(physical.cache_key);
                    }
                }

                for key in glyph_keys {
                    let _ = self.get_or_insert_glyph(key);
                }

                (max_width, total_height)
            }
            _ => (0.0, 0.0),
        }
    }

    pub fn get_or_insert_glyph(&mut self, key: CacheKey) -> Option<CachedGlyph> {
        if let Some(cached) = self.cache.get(&key) {
            return *cached;
        }

        let image = self
            .swash_cache
            .get_image_uncached(&mut self.font_system, key)?;
        let width = image.placement.width;
        let height = image.placement.height;

        if width == 0 || height == 0 {
            self.cache.insert(key, None);
            return None;
        }

        let (glyph_x, glyph_y) = match self.atlas.allocate(width, height) {
            Some(pos) => pos,
            None => {
                self.grow_atlas()?;
                self.atlas.allocate(width, height)?
            }
        };

        self.write_glyph(glyph_x, glyph_y, width, height, image.content, &image.data);
        self.atlas
            .extrude_padding(glyph_x, glyph_y, width as i32, height as i32);

        let inv_size = 1.0 / self.atlas.size() as f32;
        let uv_min = [glyph_x as f32 * inv_size, glyph_y as f32 * inv_size];
        let uv_max = [
            (glyph_x + width as i32) as f32 * inv_size,
            (glyph_y + height as i32) as f32 * inv_size,
        ];

        let cached = CachedGlyph {
            uv_min,
            uv_max,
            left: image.placement.left,
            top: image.placement.top,
            width,
            height,
        };

        self.cache.insert(key, Some(cached));
        Some(cached)
    }

    fn write_glyph(
        &mut self,
        dst_x: i32,
        dst_y: i32,
        width: u32,
        height: u32,
        content: SwashContent,
        data: &[u8],
    ) {
        let atlas_size = self.atlas.size() as usize;
        let width = width as usize;
        let height = height as usize;
        let pixels = self.atlas.pixels_mut();

        for row in 0..height {
            for col in 0..width {
                let dx = dst_x as usize + col;
                let dy = dst_y as usize + row;
                let dst_idx = (dy * atlas_size + dx) * 4;

                match content {
                    SwashContent::Mask => {
                        let src_idx = row * width + col;
                        let alpha = data.get(src_idx).copied().unwrap_or(0);
                        pixels[dst_idx..dst_idx + 4].copy_from_slice(&[255, 255, 255, alpha]);
                    }
                    SwashContent::Color => {
                        let src_idx = (row * width + col) * 4;
                        if let Some(pixel) = data.get(src_idx..src_idx + 4) {
                            pixels[dst_idx..dst_idx + 4].copy_from_slice(pixel);
                        }
                    }
                    SwashContent::SubpixelMask => {
                        let src_idx = (row * width + col) * 3;
                        let r = data.get(src_idx).copied().unwrap_or(0);
                        let g = data.get(src_idx + 1).copied().unwrap_or(0);
                        let b = data.get(src_idx + 2).copied().unwrap_or(0);
                        let alpha = r.max(g).max(b);
                        pixels[dst_idx..dst_idx + 4].copy_from_slice(&[255, 255, 255, alpha]);
                    }
                }
            }
        }
    }

    fn grow_atlas(&mut self) -> Option<()> {
        let new_size = self.atlas.size().checked_mul(2)?;
        if new_size > 8192 {
            return None;
        }

        let active_keys: Vec<CacheKey> = self.cache.keys().copied().collect();
        self.atlas.reset(new_size);
        self.cache.clear();

        for key in active_keys {
            self.get_or_insert_glyph(key);
        }

        Some(())
    }
}
