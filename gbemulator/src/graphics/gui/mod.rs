mod controls_window;
mod main_menu;
pub mod emulator_app;
pub mod palette_window;

pub struct State {
    controls_window_shown: bool,
    palette_window_shown: bool
}

impl State {
    pub fn new() -> Self {
        State {
            controls_window_shown: false,
            palette_window_shown: false
        }
    }
}
