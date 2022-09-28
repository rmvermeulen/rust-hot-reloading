extern crate shared;

use shared::State;

#[no_mangle]
pub fn update_state(delta: f64, state: &mut State) {
    state.updates += 1;

    state.actors.iter_mut().for_each(|actor| {
        actor.pos.x = (actor.pos.x + 100. * delta) % 512.;
        actor.pos.y = 400.;
    })
}
