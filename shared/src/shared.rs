#[derive(Debug, Clone)]
pub struct State {
    pub x: f64,
    pub y: f64,
    pub items: Vec<i32>,
}

impl State {
    pub fn new() -> State {
        State {
            x: 100.,
            y: 100.,
            items: Default::default(),
        }
    }
}
