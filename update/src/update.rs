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
            let actor = state.actors.get(&id).expect("Missing actor for sprite");
            let (x, y) = sprite.get_position();
            let Vec2f { x: vx, y: vy } = actor.velocity;
            sprite.set_position(x + vx, y + vy);
        }
    }
}
