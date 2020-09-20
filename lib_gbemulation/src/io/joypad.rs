use crate::util::binary::is_bit_set;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum Key {
    A,
    B,
    Left,
    Right,
    Up,
    Down,
    Start,
    Select,
}

pub struct Joypad {
    direction_key_status: u8,
    button_key_status: u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            direction_key_status: 0xFF,
            button_key_status: 0xFF,
        }
    }

    pub fn push_key(&mut self, key: Key) {
        match key {
            Key::A => self.button_key_status &= 0x01 ^ 0xF,
            Key::B => self.button_key_status &= 0x02 ^ 0xF,
            Key::Select => self.button_key_status &= 0x04 ^ 0xF,
            Key::Start => self.button_key_status &= 0x08 ^ 0xF,
            Key::Right => self.direction_key_status &= 0x01 ^ 0xF,
            Key::Left => self.direction_key_status &= 0x02 ^ 0xF,
            Key::Up => self.direction_key_status &= 0x04 ^ 0xF,
            Key::Down => self.direction_key_status &= 0x08 ^ 0xF,
        }
    }

    pub fn release_key(&mut self, key: Key) {
        match key {
            Key::A => self.button_key_status |= 0x01,
            Key::B => self.button_key_status |= 0x02,
            Key::Select => self.button_key_status |= 0x04,
            Key::Start => self.button_key_status |= 0x08,
            Key::Right => self.direction_key_status |= 0x01,
            Key::Left => self.direction_key_status |= 0x02,
            Key::Up => self.direction_key_status |= 0x04,
            Key::Down => self.direction_key_status |= 0x08,
        }
    }

    pub fn read_input(&self, value: u8) -> u8 {
        //Bit 4 = Direction keys selected
        if !is_bit_set(&value, 4) {
            return self.direction_key_status;
        }

        //Bit 5 = Button keys
        if !is_bit_set(&value, 5) {
            return self.button_key_status;
        }

        return 0;
    }
}
