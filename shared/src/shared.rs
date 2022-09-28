extern crate piston_window;
extern crate sprite;
extern crate uuid;

pub mod glyph_cache_manager;

use piston_window::G2dTexture;
use sprite::Scene;
use std::{collections::HashMap, rc::Rc, time::Duration};
use uuid::Uuid;

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
    pub id: Uuid,
    pub velocity: Vec2f,
}
impl Actor {
    pub fn new(id: Uuid) -> Actor {
        Actor {
            id,
            velocity: Vec2f { x: 0., y: 0. },
        }
    }
}

pub struct State {
    pub updates: i32,
    pub elapsed: Duration,
    pub scenes: Vec<Box<Scene<G2dTexture>>>,
    pub actors: HashMap<Uuid, Box<Actor>>,
}

impl State {
    pub fn new() -> State {
        State {
            updates: 0,
            elapsed: Default::default(),
            scenes: Default::default(),
            actors: Default::default(),
        }
    }
}
