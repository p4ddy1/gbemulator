use crate::config::config::Config;

use crate::emulation::Emulation;
use crate::graphics::gui::controls_window::ControlsWindow;
use crate::graphics::gui::main_menu::MainMenu;
use imgui::Ui;
use std::sync::{Arc, RwLock};
use winit::event::KeyboardInput;

mod controls_window;
mod main_menu;

pub struct Gui<'a> {
    main_menu: MainMenu<'a>,
    controls_window: ControlsWindow,
    state: State,
    keyboard_input: Option<KeyboardInput>,
}

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

pub trait UiElement {
    fn render(&mut self, ui: &mut Ui, state: &mut State, keyboard_input: &Option<KeyboardInput>);
}

impl<'a> Gui<'a> {
    pub fn new(config: Arc<RwLock<Config>>, emulation: &'a Emulation) -> Self {
        Gui {
            main_menu: MainMenu::new(emulation),
            controls_window: ControlsWindow::new(config),
            state: State::new(),
            keyboard_input: None,
        }
    }

    pub fn set_keyboard_input(&mut self, keyboard_input: KeyboardInput) {
        self.keyboard_input = Some(keyboard_input);
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.main_menu
            .render(ui, &mut self.state, &self.keyboard_input);
        self.controls_window
            .render(ui, &mut self.state, &self.keyboard_input);
        self.keyboard_input = None;
    }
}
