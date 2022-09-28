extern crate piston_window;
extern crate shared;
use piston_window::*;

#[no_mangle]
pub fn view_state(state: &shared::State, ctx: Context, g: &mut G2d) {
    println!("view_state: {:?}", state);
    clear([0., 0., 1., 1.], g);
    rectangle(
        [0., 0., 0., 1.],
        [-50., -50., 100., 100.],
        ctx.transform.clone().trans(state.x, state.y),
        g,
    );
    state.items.iter().enumerate().for_each(|(i, x)| {
        let intensity = *x as f32 / 1024.0;
        rectangle(
            [1.0, 0.0, 0.5, 0.5 + intensity],
            [250.0 + *x as f64, 100.0 + (i as f64) * 25.0, 20.0, 20.0],
            ctx.transform,
            g,
        );
    });

    if !state.items.is_empty() {
        print!("[");
        let mut first = true;
        for &item in state.items.iter() {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            print!("{0}", item);
        }
        print!("]");
        println!();
    }
}
