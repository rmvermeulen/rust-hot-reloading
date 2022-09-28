pub mod glyph_cache_manager;

use std::time::Duration;

pub type GlyphCacheManager = glyph_cache_manager::GlyphCacheManager;

pub struct Resources {
    pub glyphs: GlyphCacheManager,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}
impl Vec2f {
    pub const ZERO: Vec2f = Vec2f { x: 0.0, y: 0.0 };
    pub const UNIT: Vec2f = Vec2f { x: 1.0, y: 1.0 };
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub pos: Vec2f,
    pub scale: Vec2f,
}
#[derive(Debug, Clone)]
pub struct State {
    pub updates: i32,
    pub elapsed: Duration,
    pub actors: Vec<Actor>,
}

impl State {
    pub fn new() -> State {
        State {
            updates: 0,
            elapsed: Default::default(),
            actors: vec![Actor {
                pos: Vec2f { x: 100., y: 100. },
                scale: Vec2f::UNIT,
            }],
        }
    }
}
