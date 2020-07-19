use crate::graphics::gui::{State, UiElement};
use imgui::{im_str, MenuItem, Ui};
use winit::event::KeyboardInput;

pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {}
    }
}

impl UiElement for MainMenu {
    fn render(&mut self, ui: &mut Ui, state: &mut State, _: &Option<KeyboardInput>) {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                menu.end(&ui);
            }

            if let Some(menu) = ui.begin_menu(im_str!("Options"), true) {
                self.show_options_menu(ui, state);
                menu.end(&ui);
            }

            menu_bar.end(&ui);
        }
    }
}

impl MainMenu {
    fn show_options_menu(&mut self, ui: &mut Ui, state: &mut State) {
        MenuItem::new(im_str!("Controls")).build_with_ref(ui, &mut state.controls_window_shown);
    }
}
