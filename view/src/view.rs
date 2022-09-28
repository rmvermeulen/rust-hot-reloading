extern crate piston_window;
extern crate shared;
use std::path::PathBuf;

use piston_window::*;

#[no_mangle]
pub fn view_state(state: &shared::State, res: &mut shared::Resources, ctx: Context, g: &mut G2d) {
    // println!("view_state: {:?}", state);
    // clear([0., 0., 1., 1.], g);

    res.glyphs.get_glyphs_cache(0, |font| {
        text(
            [0., 0., 0., 1.],
            32,
            "text from view::view_state!",
            font,
            ctx.transform.clone().trans(400., 400.),
            g,
        )
        .expect("Failed to draw text in view::view_state")
    });

    for actor in &state.actors {
        rectangle(
            [1., 1., 0., 1.],
            [-50., -50., 100., 100.],
            ctx.transform.clone().trans(actor.pos.x, actor.pos.y),
            g,
        );
    }
}
