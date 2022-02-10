mod controls_window;
mod main_menu;
pub mod emulator_app;

pub struct State {
    controls_window_shown: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            controls_window_shown: false,
        }
    }
}
