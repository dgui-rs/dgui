use std::collections::HashMap;

use etagere::{BucketedAtlasAllocator, size2};
use swash::CacheKey;

use crate::text::CachedGlyph;

pub const ATLAS_PADDING: i32 = 2;

pub struct Atlas {
    pub allocator: BucketedAtlasAllocator,
    pub data: Vec<u8>,
    pub cache: HashMap<CacheKey, CachedGlyph>,
    pub size: i32,
}

impl Atlas {
    pub fn new() -> Self {
        let size = 1024;

        Atlas {
            allocator: BucketedAtlasAllocator::new(size2(size, size)),
            data: vec![0; (size * size * 4) as usize],
            cache: HashMap::new(),
            size,
        }
    }

    pub fn clear(&mut self) {
        self.allocator.clear();
        self.data = vec![0; (self.size * self.size * 4) as usize];
        self.cache.clear();
    }

    pub fn grow(&mut self) {
        if self.size < 4096 {
            let size = self.size * 2;

            self.allocator.grow(size2(size, size));
        } else {
            self.clear();
        }
    }
}
