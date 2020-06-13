use core::fmt;
use lib_gbemulation::io::joypad::Key;
use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Keyboard,
    Gamepad,
}

#[derive(Debug)]
pub struct KeyboardMap {
    pub map: HashMap<VirtualKeyCode, Key>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Controls {
    selected_type: Type,
    keyboard_map: KeyboardMap,
}

impl Default for Controls {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(VirtualKeyCode::W, Key::Up);
        map.insert(VirtualKeyCode::A, Key::Left);
        map.insert(VirtualKeyCode::S, Key::Down);
        map.insert(VirtualKeyCode::D, Key::Right);
        map.insert(VirtualKeyCode::LShift, Key::B);
        map.insert(VirtualKeyCode::Space, Key::A);
        map.insert(VirtualKeyCode::Return, Key::Start);
        map.insert(VirtualKeyCode::K, Key::Select);

        let keyboard_map = KeyboardMap { map };

        Controls {
            selected_type: Type::Keyboard,
            keyboard_map,
        }
    }
}

impl Serialize for KeyboardMap {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;

        let key_to_string_map = create_key_to_string_map();

        for (keyboard_key, gameboy_key) in &self.map {
            map.serialize_entry(key_to_string_map.get(&gameboy_key).unwrap(), keyboard_key);
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for KeyboardMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(KeyboardMapVisitor)
    }
}

struct KeyboardMapVisitor;

impl<'de> Visitor<'de> for KeyboardMapVisitor {
    type Value = KeyboardMap;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("Keyboard Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keyboard_map: HashMap<VirtualKeyCode, Key> =
            HashMap::with_capacity(map.size_hint().unwrap_or(0));
        let key_to_string_map = create_key_to_string_map();

        while let Some((gameboy_key, keyboard_button)) =
            map.next_entry::<String, VirtualKeyCode>()?
        {
            let gameboy_key = get_key_by_string(&key_to_string_map, gameboy_key);
            keyboard_map.insert(keyboard_button, gameboy_key);
        }

        Ok(KeyboardMap { map: keyboard_map })
    }
}

fn create_key_to_string_map() -> HashMap<Key, String> {
    let mut map = HashMap::new();
    map.insert(Key::A, "A".to_string());
    map.insert(Key::B, "B".to_string());
    map.insert(Key::Left, "Left".to_string());
    map.insert(Key::Right, "Right".to_string());
    map.insert(Key::Up, "Up".to_string());
    map.insert(Key::Down, "Down".to_string());
    map.insert(Key::Start, "Start".to_string());
    map.insert(Key::Select, "Select".to_string());
    map
}

fn get_key_by_string(map: &HashMap<Key, String>, button_string: String) -> Key {
    let key = map
        .iter()
        .find(|(_, button)| **button == button_string)
        .unwrap()
        .0;
    (*key).clone()
}
