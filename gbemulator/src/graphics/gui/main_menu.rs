use crate::graphics::gui::State;
use std::sync::mpsc::Sender;
use std::thread;

pub struct MainMenu {
    rom_filename_sender: Sender<Option<String>>,
}

impl MainMenu {
    pub fn new(rom_filename_sender: Sender<Option<String>>) -> Self {
        MainMenu {
            rom_filename_sender,
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let filename_sender = self.rom_filename_sender.clone();
                    //Thread is required otherwise this will crash on Windows TODO: Check if this is still true
                    thread::spawn(move || {
                        let filename = tinyfiledialogs::open_file_dialog(
                            "Open",
                            "",
                            Some((&["*.gb"], "Gameboy ROM")),
                        );
                        filename_sender.send(filename).unwrap();
                    });
                    ui.close_menu();
                }
            });

            ui.menu_button("Options", |ui| {
                if ui.button("Controls").clicked() {
                    state.controls_window_shown = true;
                    ui.close_menu();
                }

                if ui.button("Palette").clicked() {
                    state.palette_window_shown = true;
                    ui.close_menu();
                }
            });
        });
    }
}
