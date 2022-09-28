extern crate shared;

use shared::State;

#[no_mangle]
pub fn update_state(delta: f64, state: &mut State) {
    state.x += 100. * delta;
    state.x %= 500.;
    let n = match state.items.last() {
        Some(x) => x + (delta * 1000.0) as i32,
        None => 0,
    };

    state.items.push(n);

    while state.items.len() > 10 {
        state.items.remove(0);
    }

    state.items = state.items.iter().map(|n| n % 512).collect();
}
