extern crate piston_window;
extern crate sprite;

extern crate shared;

use piston_window::*;
use sprite::{Scene, Sprite};

#[no_mangle]
pub fn view_state(state: &shared::State, res: &mut shared::GameAssets, ctx: Context, g: &mut G2d) {
    // println!("view_state: {:?}", state);
    clear([0.7, 0.7, 0.7, 1.], g);

    res.glyphs.get_glyphs_cache(0, |font| {
        text(
            [0.2, 0.2, 0.2, 1.],
            32,
            "text from view::view_state!",
            font,
            ctx.transform.clone().trans(400., 400.),
            g,
        )
        .expect("Failed to draw text in view::view_state")
    });

    if let Some(scene) = &state.scenes.first() {
        scene.draw(ctx.transform, g);
    }
}
