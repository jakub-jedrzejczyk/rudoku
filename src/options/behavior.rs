pub struct BehaviorOptions {
    show_invalid: bool,
}

impl BehaviorOptions {
    pub fn new() -> BehaviorOptions {
        return BehaviorOptions {
            show_invalid: false,
        };
    }

    pub fn get_show_invalid(&self) -> bool {
        self.show_invalid
    }

    pub fn toggle_show_invalid(&mut self) {
        self.show_invalid = !self.show_invalid;
    }
}
