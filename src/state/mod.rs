pub struct State {
    initial_val: u8,
    current_val: u8,
}
impl State {
    fn new() -> Self {
        Self {
            initial_val: 100,
            current_val: 100,
        }
    }
}
