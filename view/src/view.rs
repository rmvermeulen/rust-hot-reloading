extern crate shared;

#[no_mangle]
pub fn view_state(state: &shared::State) {
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
