use crate::config::config::Config;
use crate::config::controls::KeyboardMap;
use crate::graphics::gui::State;

use lib_gbemulation::io::joypad::Key;
use std::sync::{Arc, RwLock};
use winit::event::{ElementState, KeyboardInput};

pub struct ControlsWindow {
    config: Arc<RwLock<Config>>,
    text_input_a: String,
    text_input_b: String,
    text_input_up: String,
    text_input_down: String,
    text_input_left: String,
    text_input_right: String,
    text_input_start: String,
    text_input_select: String,
}

impl ControlsWindow {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let cloned_config = Arc::clone(&config);
        let map = &cloned_config.read().unwrap().controls.keyboard_map;

        ControlsWindow {
            config,
            text_input_a: parse_key_mapping(&map, Key::A),
            text_input_b: parse_key_mapping(&map, Key::B),
            text_input_up: parse_key_mapping(&map, Key::Up),
            text_input_down: parse_key_mapping(&map, Key::Down),
            text_input_left: parse_key_mapping(&map, Key::Left),
            text_input_right: parse_key_mapping(&map, Key::Right),
            text_input_start: parse_key_mapping(&map, Key::Start),
            text_input_select: parse_key_mapping(&map, Key::Select),
        }
    }

    pub fn update(
        &mut self,
        ctx: &egui::CtxRef,
        state: &mut State,
        keyboard_input: Option<KeyboardInput>,
    ) {
        egui::Window::new("Controls")
            .open(&mut state.controls_window_shown)
            .show(ctx, |ui| {
                ui.columns(4, |ui| {
                    let col1 = ui.get_mut(0).unwrap();
                    create_keyboard_input_field(
                        &self.config,
                        col1,
                        &mut self.text_input_a,
                        keyboard_input,
                        Key::A,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col1,
                        &mut self.text_input_up,
                        keyboard_input,
                        Key::Up,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col1,
                        &mut self.text_input_left,
                        keyboard_input,
                        Key::Left,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col1,
                        &mut self.text_input_start,
                        keyboard_input,
                        Key::Start,
                    );

                    let col2 = ui.get_mut(1).unwrap();

                    create_clear_button(&self.config, col2, &mut self.text_input_a, Key::A);
                    create_clear_button(&self.config, col2, &mut self.text_input_up, Key::Up);
                    create_clear_button(&self.config, col2, &mut self.text_input_left, Key::Left);
                    create_clear_button(&self.config, col2, &mut self.text_input_start, Key::Start);

                    let col3 = ui.get_mut(2).unwrap();

                    create_keyboard_input_field(
                        &self.config,
                        col3,
                        &mut self.text_input_b,
                        keyboard_input,
                        Key::B,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col3,
                        &mut self.text_input_down,
                        keyboard_input,
                        Key::Down,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col3,
                        &mut self.text_input_right,
                        keyboard_input,
                        Key::Right,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        col3,
                        &mut self.text_input_select,
                        keyboard_input,
                        Key::Select,
                    );

                    let col4 = ui.get_mut(3).unwrap();

                    create_clear_button(&self.config, col4, &mut self.text_input_b, Key::B);
                    create_clear_button(&self.config, col4, &mut self.text_input_down, Key::Down);
                    create_clear_button(&self.config, col4, &mut self.text_input_right, Key::Right);
                    create_clear_button(
                        &self.config,
                        col4,
                        &mut self.text_input_select,
                        Key::Select,
                    );
                });
            });
    }
}

fn create_keyboard_input_field(
    config: &Arc<RwLock<Config>>,
    ui: &mut egui::Ui,
    text_input: &mut String,
    keyboard_input: Option<KeyboardInput>,
    key: Key,
) {
    ui.label(format!("{:?}", key));

    let text_edit = ui.text_edit_singleline(text_input);
    if text_edit.has_focus() {
        if let Some(input) = keyboard_input {
            if input.state == ElementState::Pressed {
                let map = &mut config.write().unwrap().controls.keyboard_map;
                if let Some(virtual_key_code) = input.virtual_keycode {
                    map.add_key_code_by_key(key, &virtual_key_code);
                    *text_input = parse_key_mapping(map, key);
                }
            }
        }
    }
}

fn create_clear_button(
    config: &Arc<RwLock<Config>>,
    ui: &mut egui::Ui,
    text_input: &mut String,
    key: Key,
) {
    ui.label("");
    if ui.button("[X]").clicked() {
        let map = &mut config.write().unwrap().controls.keyboard_map;
        map.clear_mapping_by_key(key);
        *text_input = parse_key_mapping(map, key);
    }
}

fn parse_key_mapping(map: &KeyboardMap, key: Key) -> String {
    format!("{:?}", map.get_key_code_by_key(key))
}
