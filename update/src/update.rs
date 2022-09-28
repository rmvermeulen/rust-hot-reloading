extern crate shared;

use std::time::Duration;

use shared::{State, Vec2f};

#[no_mangle]
pub fn update_state(delta: f64, state: &mut State) {
    state.updates += 1;
    state.elapsed += Duration::from_secs_f64(delta);

    if let Some(scene) = &mut state.scenes.first_mut() {
        let mut ids = vec![];
        for sprite in scene.children() {
            ids.push(sprite.id());
        }
        for id in ids {
            let sprite = scene
                .child_mut(id)
                .expect("Sprite suddenly missing? Scene changed somehow?");
            let actor = state.actors.get_mut(&id).expect("Missing actor for sprite");
            let (x, y) = sprite.get_position();
            let v = &mut actor.velocity;
            let x = x + v.x * delta;
            let y = y + v.y * delta;
            sprite.set_position(x % 512., y % 512.);
            v.y += 100. * delta;
            v.y = if v.y > 1000. {
                1000.
            } else if v.y < -1000. {
                -1000.
            } else {
                v.y
            };
        }
    }
}
