use crate::config::config::Config;
use crate::graphics::gui::controls_window::ControlsWindow;
use crate::graphics::gui::main_menu::MainMenu;
use crate::graphics::gui::palette_window::PaletteWindow;
use crate::graphics::gui::State;
use egui::{CtxRef, TextureId};
use epi::Frame;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use winit::event::KeyboardInput;

pub struct EmulatorApp {
    main_menu: MainMenu,
    controls_window: ControlsWindow,
    palette_window: PaletteWindow,
    state: State,
    keyboard_input: Option<KeyboardInput>,
    tex: Option<TextureId>,
}

impl EmulatorApp {
    pub fn new(rom_filename_sender: Sender<Option<String>>, config: &Arc<RwLock<Config>>) -> Self {
        EmulatorApp {
            main_menu: MainMenu::new(rom_filename_sender),
            controls_window: ControlsWindow::new(config.clone()),
            palette_window: PaletteWindow::new(config.clone()),
            state: State::new(),
            keyboard_input: None,
            tex: None,
        }
    }

    pub fn set_keyboard_input(&mut self, keyboard_input: KeyboardInput) {
        self.keyboard_input = Some(keyboard_input);
    }

    pub fn set_tex(&mut self, tex: TextureId) {
        self.tex = Some(tex);
    }
}

impl epi::App for EmulatorApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            self.main_menu.update(ui, &mut self.state);
        });

        self.controls_window
            .update(ctx, &mut self.state, self.keyboard_input);
        self.palette_window.update(ctx, &mut self.state);

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tex {
                Some(t) => {
                    ui.image(t, ui.available_size());
                }
                None => {}
            };
        });
    }

    fn name(&self) -> &str {
        return "emulator_app";
    }
}
