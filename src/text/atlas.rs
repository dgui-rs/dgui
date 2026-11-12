use std::collections::HashMap;

use cosmic_text::CacheKey;
use etagere::{AtlasAllocator, size2};

use crate::text::CachedGlyph;

pub const ATLAS_PADDING: i32 = 2;

pub struct Atlas {
    pub allocator: AtlasAllocator,
    pub size: i32,
    pub data: Vec<u8>,
    pub cache: HashMap<CacheKey, CachedGlyph>,
}

impl Atlas {
    pub fn grow(&mut self) {}
}
