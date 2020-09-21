use crate::config::config::Config;
use crate::config::config_storage::ConfigStorage;
use lib_gbemulation::io::joypad::Joypad;
use std::sync::{Arc, Mutex, RwLock};
use winit::event::VirtualKeyCode;

pub struct KeyboardController {
    pub joypad: Arc<Mutex<Joypad>>,
    config: Arc<RwLock<Config>>,
}

impl KeyboardController {
    pub fn new(joypad: Arc<Mutex<Joypad>>, config_storage: &ConfigStorage) -> Self {
        KeyboardController {
            joypad,
            config: Arc::clone(&config_storage.config),
        }
    }

    pub fn push_key(&self, key_code: VirtualKeyCode) {
        let config = self.config.read().unwrap();
        let mut joypad = self.joypad.lock().unwrap();
        if let Some(key_list) = config.controls.keyboard_map.map.get(&key_code) {
            for key in key_list {
                joypad.push_key(key.clone());
            }
        }
    }

    pub fn release_key(&self, key_code: VirtualKeyCode) {
        let config = self.config.read().unwrap();
        let mut joypad = self.joypad.lock().unwrap();
        if let Some(key_list) = config.controls.keyboard_map.map.get(&key_code) {
            for key in key_list {
                joypad.release_key(key.clone());
            }
        }
    }
}
