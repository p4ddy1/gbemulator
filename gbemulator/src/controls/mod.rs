use crate::config::config_storage::ConfigStorage;
use crate::controls::keyboard_receiver::KeyboardReceiver;
use crate::controls::keyboard_sender::KeyboardSender;
use lib_gbemulation::io::joypad::Key;
use std::sync::mpsc::channel;

pub mod keyboard_controller;
pub mod keyboard_receiver;
pub mod keyboard_sender;

pub enum KeyEvent {
    KeyPressed(Key),
    KeyReleased(Key),
}

pub fn new_keyboard(config_storage: &ConfigStorage) -> (KeyboardSender, KeyboardReceiver) {
    let (sender, receiver) = channel();
    (
        KeyboardSender::new(sender, config_storage),
        KeyboardReceiver::new(receiver),
    )
}
