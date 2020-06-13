use crate::config::config_storage::ConfigStorage;
use crate::config::controls::KeyboardMap;
use crate::controls::KeyEvent;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use winit::event::VirtualKeyCode;
use crate::config::config::Config;

pub struct KeyboardSender {
    sender: Sender<KeyEvent>,
    config: Arc<RwLock<Config>>,
}

impl KeyboardSender {
    pub fn new(sender: Sender<KeyEvent>, config_storage: &ConfigStorage) -> Self {
        KeyboardSender {
            sender,
            config: Arc::clone(&config_storage.config),
        }
    }

    pub fn press_key(&self, key_code: VirtualKeyCode) {
        let config = self.config.read().unwrap();
        if let Some(key) = config.controls.keyboard_map.map.get(&key_code) {
            self.sender.send(KeyEvent::KeyPressed(key.clone()));
        }
    }

    pub fn release_key(&self, key_code: VirtualKeyCode) {
        let config = self.config.read().unwrap();
        if let Some(key) = config.controls.keyboard_map.map.get(&key_code) {
            self.sender.send(KeyEvent::KeyReleased(key.clone()));
        }
    }
}
