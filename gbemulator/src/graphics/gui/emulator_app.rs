use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use egui::{CtxRef, TextureId, Vec2};
use epi::Frame;
use winit::event::KeyboardInput;
use crate::config::config::Config;
use crate::graphics::gui::controls_window::ControlsWindow;
use crate::graphics::gui::main_menu::MainMenu;
use crate::graphics::gui::State;

pub struct EmulatorApp {
    main_menu: MainMenu,
    controls_window: ControlsWindow,
    state: State,
    keyboard_input: Option<KeyboardInput>,
    tex: Option<TextureId>
}

impl EmulatorApp {
    pub fn new(rom_filename_sender: Sender<Option<String>>, config: Arc<RwLock<Config>>) -> Self {
        EmulatorApp {
            main_menu: MainMenu::new(rom_filename_sender),
            controls_window: ControlsWindow::new(config),
            state: State::new(),
            keyboard_input: None,
            tex: None
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
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        egui::TopBottomPanel::top("main_menu").show(ctx, |ui| {
            self.main_menu.update(ui, frame, &mut self.state);
        });
        self.controls_window.update(ctx, &mut self.state, self.keyboard_input);

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tex {
                Some(t) => {
                    ui.image(t, ui.available_size());
                },
                None => {}
            };
        });
    }

    fn name(&self) -> &str {
        return "emulator_app";
    }
}