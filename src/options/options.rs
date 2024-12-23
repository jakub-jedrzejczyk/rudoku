use crate::options::display::DisplayOptions;
use crate::options::behavior::BehaviorOptions;

pub struct Options {
    pub display: DisplayOptions,
    pub behavior: BehaviorOptions,
}

impl Options {
    pub fn new() -> Options {
        return Options {
            display: DisplayOptions::new(),
            behavior: BehaviorOptions::new(),
        };
    }
}
