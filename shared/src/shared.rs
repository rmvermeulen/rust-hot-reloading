extern crate piston_window;
extern crate sprite;
pub mod glyph_cache_manager;

use piston_window::G2dTexture;
use sprite::Scene;
use std::{collections::HashMap, rc::Rc, time::Duration};

pub type GlyphCacheManager = glyph_cache_manager::GlyphCacheManager;

pub struct GameAssets {
    pub glyphs: GlyphCacheManager,
    pub textures: HashMap<String, Rc<G2dTexture>>,
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
// #[derive(Debug)]
pub struct State {
    pub updates: i32,
    pub elapsed: Duration,
    pub actors: Vec<Actor>,
    pub current_scene: Option<Scene<G2dTexture>>,
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
            current_scene: None,
        }
    }
}
