use std::collections::HashMap;

use cosmic_text::{
    Attrs, Buffer, CacheKey, FontSystem, Metrics, Shaping, SwashCache, SwashContent,
};
use etagere::{AtlasAllocator, Size, size2};

use crate::{Widget, widgets::WidgetType};

const ATLAS_PADDING: i32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct CachedGlyph {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],

    pub left: i32,
    pub top: i32,

    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct DirtyRect {
    pub min: [u32; 2],
    pub max: [u32; 2],
}

impl DirtyRect {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            min: [x, y],
            max: [x + width, y + height],
        }
    }

    fn union(&mut self, other: Self) {
        self.min[0] = self.min[0].min(other.min[0]);
        self.min[1] = self.min[1].min(other.min[1]);
        self.max[0] = self.max[0].max(other.max[0]);
        self.max[1] = self.max[1].max(other.max[1]);
    }

    pub fn width(&self) -> u32 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> u32 {
        self.max[1] - self.min[1]
    }
}

pub struct Text {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,

    pub atlas: AtlasAllocator,
    pub atlas_size: u32,
    pub atlas_pixels: Vec<u8>,
    pub is_dirty: bool,
    pub dirty_rect: Option<DirtyRect>,

    pub cache: HashMap<CacheKey, Option<CachedGlyph>>,
}

impl Text {
    pub fn new(atlas_size: u32) -> Self {
        assert!(atlas_size > 0, "atlas_size must be greater than zero");

        let pixel_count = Self::pixel_count(atlas_size);

        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),

            atlas: AtlasAllocator::new(Size::new(atlas_size as i32, atlas_size as i32)),

            atlas_size,
            atlas_pixels: vec![0; pixel_count],

            is_dirty: false,
            dirty_rect: None,

            cache: HashMap::new(),
        }
    }

    fn pixel_count(size: u32) -> usize {
        (size as usize)
            .checked_mul(size as usize)
            .and_then(|v| v.checked_mul(4))
            .expect("atlas is too large")
    }

    pub fn create_buffer(&mut self, font_size: f32, line_height: f32) -> Buffer {
        assert!(font_size > 0.0, "font_size must be greater than zero");
        assert!(line_height > 0.0, "line_height must be greater than zero");

        Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height))
    }

    pub fn render(&mut self, widget: &mut Widget) -> (f32, f32) {
        match &widget.type_of {
            WidgetType::Text { text } => {
                let text_str = text.get();

                let Some(buffer) = widget.buffer.as_mut() else {
                    return (0.0, 0.0);
                };

                buffer.set_text(&text_str, &Attrs::new(), Shaping::Advanced, None);

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

            WidgetType::TextInput { .. } => (0.0, 0.0),

            _ => (0.0, 0.0),
        }
    }

    pub fn get_or_insert_glyph(&mut self, key: CacheKey) -> Option<CachedGlyph> {
        if let Some(cached) = self.cache.get(&key) {
            return *cached;
        }

        let (content, placement, data) = {
            let image = self
                .swash_cache
                .get_image_uncached(&mut self.font_system, key)?;

            (image.content, image.placement, image.data.to_vec())
        };

        let width = placement.width;
        let height = placement.height;

        if width == 0 || height == 0 {
            self.cache.insert(key, None);
            return None;
        }

        let padded_width = width as i32 + ATLAS_PADDING * 2;
        let padded_height = height as i32 + ATLAS_PADDING * 2;

        let allocation = match self.atlas.allocate(size2(padded_width, padded_height)) {
            Some(allocation) => allocation,

            None => {
                self.grow_atlas()?;

                self.atlas.allocate(size2(padded_width, padded_height))?
            }
        };

        let glyph_x = allocation.rectangle.min.x + ATLAS_PADDING;

        let glyph_y = allocation.rectangle.min.y + ATLAS_PADDING;

        self.write_glyph(glyph_x, glyph_y, width, height, content, &data);

        self.extrude_padding(glyph_x, glyph_y, width as i32, height as i32);

        let dirty_x = allocation.rectangle.min.x as u32;
        let dirty_y = allocation.rectangle.min.y as u32;

        let dirty_width = allocation.rectangle.width() as u32;

        let dirty_height = allocation.rectangle.height() as u32;

        self.mark_dirty(DirtyRect::new(dirty_x, dirty_y, dirty_width, dirty_height));

        let inv_size = 1.0 / self.atlas_size as f32;

        let uv_min = [glyph_x as f32 * inv_size, glyph_y as f32 * inv_size];

        let uv_max = [
            (glyph_x + width as i32) as f32 * inv_size,
            (glyph_y + height as i32) as f32 * inv_size,
        ];

        let cached = CachedGlyph {
            uv_min,
            uv_max,

            left: placement.left,
            top: placement.top,

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
        let atlas_size = self.atlas_size as usize;

        let width = width as usize;
        let height = height as usize;

        for row in 0..height {
            for col in 0..width {
                let dst_x = dst_x as usize + col;
                let dst_y = dst_y as usize + row;

                let dst_idx = (dst_y * atlas_size + dst_x) * 4;

                match content {
                    SwashContent::Mask => {
                        let src_idx = row * width + col;

                        let alpha = data.get(src_idx).copied().unwrap_or(0);

                        self.atlas_pixels[dst_idx..dst_idx + 4]
                            .copy_from_slice(&[255, 255, 255, alpha]);
                    }

                    SwashContent::Color => {
                        let src_idx = (row * width + col) * 4;

                        if let Some(pixel) = data.get(src_idx..src_idx + 4) {
                            self.atlas_pixels[dst_idx..dst_idx + 4].copy_from_slice(pixel);
                        }
                    }

                    SwashContent::SubpixelMask => {
                        let src_idx = (row * width + col) * 3;

                        let r = data.get(src_idx).copied().unwrap_or(0);

                        let g = data.get(src_idx + 1).copied().unwrap_or(0);

                        let b = data.get(src_idx + 2).copied().unwrap_or(0);

                        let alpha = r.max(g).max(b);

                        self.atlas_pixels[dst_idx..dst_idx + 4]
                            .copy_from_slice(&[255, 255, 255, alpha]);
                    }
                }
            }
        }
    }

    fn extrude_padding(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let atlas_size = self.atlas_size as i32;

        for row in 0..height {
            let row_y = y + row;

            for padding in 1..=ATLAS_PADDING {
                self.copy_pixel(x, row_y, x - padding, row_y);

                self.copy_pixel(x + width - 1, row_y, x + width - 1 + padding, row_y);
            }
        }

        for col in -ATLAS_PADDING..width + ATLAS_PADDING {
            let src_x = x + col;

            if src_x < 0 || src_x >= atlas_size {
                continue;
            }

            for padding in 1..=ATLAS_PADDING {
                self.copy_pixel(src_x, y, src_x, y - padding);

                self.copy_pixel(src_x, y + height - 1, src_x, y + height - 1 + padding);
            }
        }
    }

    fn copy_pixel(&mut self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32) {
        let size = self.atlas_size as i32;

        if src_x < 0
            || src_y < 0
            || dst_x < 0
            || dst_y < 0
            || src_x >= size
            || src_y >= size
            || dst_x >= size
            || dst_y >= size
        {
            return;
        }

        let size = size as usize;

        let src_idx = (src_y as usize * size + src_x as usize) * 4;

        let dst_idx = (dst_y as usize * size + dst_x as usize) * 4;

        let pixel = [
            self.atlas_pixels[src_idx],
            self.atlas_pixels[src_idx + 1],
            self.atlas_pixels[src_idx + 2],
            self.atlas_pixels[src_idx + 3],
        ];

        self.atlas_pixels[dst_idx..dst_idx + 4].copy_from_slice(&pixel);
    }

    fn mark_dirty(&mut self, rect: DirtyRect) {
        self.is_dirty = true;

        match &mut self.dirty_rect {
            Some(existing) => existing.union(rect),
            None => self.dirty_rect = Some(rect),
        }
    }

    pub fn take_dirty_rect(&mut self) -> Option<DirtyRect> {
        self.dirty_rect.take()
    }

    pub fn clear_dirty(&mut self) {
        self.is_dirty = false;
        self.dirty_rect = None;
    }

    fn grow_atlas(&mut self) -> Option<()> {
        let new_size = self.atlas_size.checked_mul(2)?;
        if new_size > 8192 {
            return None;
        }

        let active_keys: Vec<CacheKey> = self.cache.keys().copied().collect();

        self.atlas_size = new_size;
        self.atlas = AtlasAllocator::new(Size::new(new_size as i32, new_size as i32));
        self.atlas_pixels = vec![0; Self::pixel_count(new_size)];
        self.cache.clear();

        for key in active_keys {
            self.get_or_insert_glyph(key);
        }

        self.is_dirty = true;
        self.dirty_rect = Some(DirtyRect::new(0, 0, new_size, new_size));

        Some(())
    }

    pub fn atlas_texture_size(&self) -> u32 {
        self.atlas_size
    }

    pub fn atlas_data(&self) -> &[u8] {
        &self.atlas_pixels
    }
}
