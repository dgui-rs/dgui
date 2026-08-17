use etagere::{AtlasAllocator, Size, size2};

pub const ATLAS_PADDING: i32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct DirtyRect {
    pub min: [u32; 2],
    pub max: [u32; 2],
}

impl DirtyRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            min: [x, y],
            max: [x + width, y + height],
        }
    }

    pub fn union(&mut self, other: Self) {
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

pub struct Atlas {
    allocator: AtlasAllocator,
    size: u32,
    pixels: Vec<u8>,
    pub dirty_rect: Option<DirtyRect>,
}

impl Atlas {
    pub fn new(size: u32) -> Self {
        assert!(size > 0, "atlas_size must be > 0");
        Self {
            allocator: AtlasAllocator::new(Size::new(size as i32, size as i32)),
            size,
            pixels: vec![0; (size as usize) * (size as usize) * 4],
            dirty_rect: None,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn allocate(&mut self, width: u32, height: u32) -> Option<(i32, i32)> {
        let padded_w = width as i32 + ATLAS_PADDING * 2;
        let padded_h = height as i32 + ATLAS_PADDING * 2;

        let alloc = self.allocator.allocate(size2(padded_w, padded_h))?;
        let gx = alloc.rectangle.min.x + ATLAS_PADDING;
        let gy = alloc.rectangle.min.y + ATLAS_PADDING;

        let dirty = DirtyRect::new(
            alloc.rectangle.min.x as u32,
            alloc.rectangle.min.y as u32,
            alloc.rectangle.width() as u32,
            alloc.rectangle.height() as u32,
        );
        self.mark_dirty(dirty);

        Some((gx, gy))
    }

    pub fn reset(&mut self, new_size: u32) {
        self.size = new_size;
        self.allocator = AtlasAllocator::new(Size::new(new_size as i32, new_size as i32));
        self.pixels = vec![0; (new_size as usize) * (new_size as usize) * 4];
        self.dirty_rect = Some(DirtyRect::new(0, 0, new_size, new_size));
    }

    pub fn extrude_padding(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let size = self.size as i32;

        for row in 0..height {
            let row_y = y + row;
            for padding in 1..=ATLAS_PADDING {
                self.copy_pixel(x, row_y, x - padding, row_y);
                self.copy_pixel(x + width - 1, row_y, x + width - 1 + padding, row_y);
            }
        }

        for col in -ATLAS_PADDING..width + ATLAS_PADDING {
            let src_x = x + col;
            if src_x < 0 || src_x >= size {
                continue;
            }
            for padding in 1..=ATLAS_PADDING {
                self.copy_pixel(src_x, y, src_x, y - padding);
                self.copy_pixel(src_x, y + height - 1, src_x, y + height - 1 + padding);
            }
        }
    }

    fn copy_pixel(&mut self, src_x: i32, src_y: i32, dst_x: i32, dst_y: i32) {
        let size = self.size as i32;
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

        let stride = self.size as usize;
        let src_idx = (src_y as usize * stride + src_x as usize) * 4;
        let dst_idx = (dst_y as usize * stride + dst_x as usize) * 4;

        let pixel = [
            self.pixels[src_idx],
            self.pixels[src_idx + 1],
            self.pixels[src_idx + 2],
            self.pixels[src_idx + 3],
        ];
        self.pixels[dst_idx..dst_idx + 4].copy_from_slice(&pixel);
    }

    pub fn mark_dirty(&mut self, rect: DirtyRect) {
        match &mut self.dirty_rect {
            Some(existing) => existing.union(rect),
            None => self.dirty_rect = Some(rect),
        }
    }

    pub fn take_dirty_rect(&mut self) -> Option<DirtyRect> {
        self.dirty_rect.take()
    }
}
