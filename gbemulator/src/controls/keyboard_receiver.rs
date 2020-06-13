use crate::controls::KeyEvent;
use lib_gbemulation::io::joypad::Joypad;
use std::sync::mpsc::Receiver;

pub struct KeyboardReceiver {
    receiver: Receiver<KeyEvent>,
}

impl KeyboardReceiver {
    pub fn new(receiver: Receiver<KeyEvent>) -> Self {
        KeyboardReceiver { receiver }
    }

    pub fn receive(&self, joypad: &mut Joypad) {
        for event in self.receiver.try_iter() {
            match event {
                KeyEvent::KeyPressed(key) => joypad.push_key(key),
                KeyEvent::KeyReleased(key) => joypad.release_key(key),
            }
        }
    }
}
