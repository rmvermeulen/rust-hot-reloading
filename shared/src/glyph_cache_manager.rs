extern crate piston_window;

use self::piston_window::*;
use std::path::PathBuf;

pub struct GlyphCacheManager {
    glyphs: Vec<Box<Glyphs>>,
}

impl GlyphCacheManager {
    pub fn new(
        window: &mut PistonWindow,
        assets_path: &PathBuf,
        fonts: Vec<&str>,
    ) -> GlyphCacheManager {
        GlyphCacheManager {
            glyphs: fonts
                .into_iter()
                .map(|font| window.load_font(assets_path.join(font)).map(Box::new))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
        }
    }

    pub fn get_glyphs_cache<F>(&mut self, index: usize, mut use_fn: F)
    where
        F: FnMut(&mut Glyphs) -> (),
    {
        self.glyphs.get_mut(index).map(|g| use_fn(&mut *g));
    }
    pub fn flush_factory_encoder(&mut self, device: &mut GfxDevice) {
        self.glyphs
            .iter_mut()
            .for_each(|g| g.factory.encoder.flush(device));
    }
}
