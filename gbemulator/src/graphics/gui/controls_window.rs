use crate::config::config::Config;
use crate::graphics::gui::{State, UiElement};
use imgui::{im_str, Condition, ImGuiInputTextFlags, ImString, InputText, Ui, Window};
use lib_gbemulation::io::joypad::Key;
use serde::export::Option::Some;
use std::sync::{Arc, RwLock};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

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
            text_input_a: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::A))),
            text_input_b: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::B))),
            text_input_up: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::Up))),
            text_input_down: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::Down))),
            text_input_left: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::Left))),
            text_input_right: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::Right))),
            text_input_start: ImString::from(format!("{:?}", map.get_key_code_by_key(Key::Start))),
            text_input_select: ImString::from(format!(
                "{:?}",
                map.get_key_code_by_key(Key::Select)
            )),
        }
    }
}

impl UiElement for ControlsWindow {
    fn render(&mut self, ui: &mut Ui, state: &mut State, keyboard_input: &Option<KeyboardInput>) {
        if state.controls_window_shown {
            Window::new(im_str!("Controls"))
                .size([350.0, 350.0], Condition::FirstUseEver)
                .opened(&mut state.controls_window_shown)
                .build(ui, || {
                    ui.columns(2, im_str!("Keyboard Buttons"), false);
                    ui.input_text(im_str!("A"), &mut self.text_input_a)
                        .read_only(true)
                        .build();

                    if ui.is_item_active() {
                        if let Some(input) = keyboard_input {
                            if input.state == ElementState::Pressed {
                                let map = &mut self.config.write().unwrap().controls.keyboard_map;
                                map.set_key_code_by_key(Key::A, &input.virtual_keycode.unwrap());
                                self.text_input_a =
                                    ImString::from(format!("{:?}", map.get_key_code_by_key(Key::A)))
                            }
                        }
                    }

                    ui.input_text(im_str!("Up"), &mut self.text_input_up)
                        .read_only(true)
                        .build();
                    ui.input_text(im_str!("Left"), &mut self.text_input_left)
                        .read_only(true)
                        .build();
                    ui.input_text(im_str!("Start"), &mut self.text_input_start)
                        .read_only(true)
                        .build();

                    ui.next_column();

                    ui.input_text(im_str!("B"), &mut self.text_input_b)
                        .read_only(true)
                        .build();
                    ui.input_text(im_str!("Down"), &mut self.text_input_down)
                        .read_only(true)
                        .build();
                    ui.input_text(im_str!("Right"), &mut self.text_input_right)
                        .read_only(true)
                        .build();
                    ui.input_text(im_str!("Select"), &mut self.text_input_select)
                        .read_only(true)
                        .build();
                });
        }
    }
}
