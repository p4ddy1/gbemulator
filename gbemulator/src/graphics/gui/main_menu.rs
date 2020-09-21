use crate::emulation::Emulation;
use crate::graphics::gui::{State, UiElement};
use crate::EmulationSignal;
use imgui::{im_str, MenuItem, Ui};
use serde::export::Option::Some;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use winit::event::KeyboardInput;

pub struct MainMenu<'a> {
    emulation: &'a Emulation,
    emulation_signal_sender: Option<Rc<Sender<EmulationSignal>>>,
}

impl<'a> MainMenu<'a> {
    pub fn new(emulation: &'a Emulation) -> Self {
        MainMenu {
            emulation,
            emulation_signal_sender: None,
        }
    }

    fn show_options_menu(&mut self, ui: &mut Ui, state: &mut State) {
        MenuItem::new(im_str!("Controls")).build_with_ref(ui, &mut state.controls_window_shown);
    }

    fn show_file_menu(&mut self, ui: &mut Ui, state: &mut State) {
        if MenuItem::new(im_str!("Open ROM")).build(ui) {
            let rom_file: String;
            match tinyfiledialogs::open_file_dialog("Open", "", Some((&["*.gb"], "Gameboy ROM"))) {
                Some(file) => rom_file = file,
                None => {
                    return;
                }
            }
            if let Some(sender) = &self.emulation_signal_sender {
                //Stop running emulation
                sender.send(EmulationSignal::Quit);
            }

            let sender = self.emulation.start(&rom_file).unwrap();

            self.emulation_signal_sender = Some(Rc::new(sender));
        }
    }
}

impl<'a> UiElement for MainMenu<'a> {
    fn render(&mut self, ui: &mut Ui, state: &mut State, _: &Option<KeyboardInput>) {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
                self.show_file_menu(ui, state);
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
