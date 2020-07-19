use crate::config::config::Config;
use crate::config::controls::KeyboardMap;
use crate::graphics::gui::{State, UiElement};

use imgui::{im_str, Condition, ImString, Ui, Window};
use lib_gbemulation::io::joypad::Key;
use serde::export::Option::Some;
use std::sync::{Arc, RwLock};
use winit::event::{ElementState, KeyboardInput};

pub struct ControlsWindow {
    config: Arc<RwLock<Config>>,
    text_input_a: ImString,
    text_input_b: ImString,
    text_input_up: ImString,
    text_input_down: ImString,
    text_input_left: ImString,
    text_input_right: ImString,
    text_input_start: ImString,
    text_input_select: ImString,
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
}

fn parse_key_mapping(map: &KeyboardMap, key: Key) -> ImString {
    ImString::from(format!("{:?}", map.get_key_code_by_key(key)))
}

fn create_keyboard_input_field(
    config: &Arc<RwLock<Config>>,
    ui: &Ui,
    keyboard_input: &Option<KeyboardInput>,
    text_input: &mut ImString,
    key: Key,
) {
    ui.input_text(&ImString::from(format!("{:?}", key)), text_input)
        .read_only(true)
        .build();

    //Listen for keyboard input and map it to
    if ui.is_item_active() {
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

fn create_clear_button(config: &Arc<RwLock<Config>>, ui: &Ui, text_input: &mut ImString, key: Key) {
    if ui.button(&ImString::from(format!("X##{:?}", key)), [0.0, 0.0]) {
        let map = &mut config.write().unwrap().controls.keyboard_map;
        map.clear_mapping_by_key(key);
        *text_input = parse_key_mapping(map, key);
    }
}

impl UiElement for ControlsWindow {
    fn render(&mut self, ui: &mut Ui, state: &mut State, keyboard_input: &Option<KeyboardInput>) {
        if state.controls_window_shown {
            Window::new(im_str!("Controls"))
                .size([500.0, 300.0], Condition::FirstUseEver)
                .opened(&mut state.controls_window_shown)
                .build(ui, || {
                    ui.columns(4, im_str!("Keyboard Buttons"), false);

                    ui.set_column_width(0, 200.0);

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_a,
                        Key::A,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_up,
                        Key::Up,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_left,
                        Key::Left,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_start,
                        Key::Start,
                    );

                    ui.next_column();
                    ui.set_column_width(1, 50.0);

                    create_clear_button(&self.config, ui, &mut self.text_input_a, Key::A);
                    create_clear_button(&self.config, ui, &mut self.text_input_up, Key::Up);
                    create_clear_button(&self.config, ui, &mut self.text_input_left, Key::Left);
                    create_clear_button(&self.config, ui, &mut self.text_input_start, Key::Start);

                    ui.next_column();
                    ui.set_column_width(2, 200.0);

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_b,
                        Key::B,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_down,
                        Key::Down,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_right,
                        Key::Right,
                    );

                    create_keyboard_input_field(
                        &self.config,
                        ui,
                        keyboard_input,
                        &mut self.text_input_select,
                        Key::Select,
                    );

                    ui.next_column();
                    ui.set_column_width(1, 50.0);

                    create_clear_button(&self.config, ui, &mut self.text_input_a, Key::B);
                    create_clear_button(&self.config, ui, &mut self.text_input_up, Key::Down);
                    create_clear_button(&self.config, ui, &mut self.text_input_left, Key::Right);
                    create_clear_button(&self.config, ui, &mut self.text_input_start, Key::Select);
                });
        }
    }
}
