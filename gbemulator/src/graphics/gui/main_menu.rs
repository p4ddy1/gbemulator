use crate::graphics::gui::{State, UiElement};
use imgui::{im_str, MenuItem, Ui};
use serde::export::Option::Some;
use std::sync::mpsc::Sender;
use std::thread;
use winit::event::KeyboardInput;

pub struct MainMenu {
    rom_filename_sender: Sender<Option<String>>,
}

impl MainMenu {
    pub fn new(rom_filename_sender: Sender<Option<String>>) -> Self {
        MainMenu {
            rom_filename_sender,
        }
    }

    fn show_options_menu(&mut self, ui: &mut Ui, state: &mut State) {
        MenuItem::new(im_str!("Controls")).build_with_ref(ui, &mut state.controls_window_shown);
    }

    fn show_file_menu(&mut self, ui: &mut Ui, _state: &mut State) {
        if MenuItem::new(im_str!("Open ROM")).build(ui) {
            let filename_sender = self.rom_filename_sender.clone();
            //Thread is required otherwise this will crash on Windows
            thread::spawn(move || {
                let filename =
                    tinyfiledialogs::open_file_dialog("Open", "", Some((&["*.gb"], "Gameboy ROM")));
                filename_sender.send(filename).unwrap();
            });
        }
    }
}

impl UiElement for MainMenu {
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
