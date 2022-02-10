use core::fmt;
use lib_gbemulation::io::joypad::Key;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use winit::event::VirtualKeyCode;

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Keyboard,
    Gamepad,
}

#[derive(Debug, Clone)]
pub struct KeyboardMap {
    pub map: HashMap<VirtualKeyCode, Vec<Key>>,
}

impl KeyboardMap {
    pub fn get_key_code_by_key(&self, key: Key) -> Vec<&VirtualKeyCode> {
        self.map
            .iter()
            .filter(|(_, gameboy_key_list)| {
                gameboy_key_list
                    .iter()
                    .find(|gameboy_key| **gameboy_key == key)
                    .is_some()
            })
            .map(|(keyboard_key, _)| keyboard_key)
            .collect()
    }

    pub fn add_key_code_by_key(&mut self, key: Key, key_code: &VirtualKeyCode) {
        if !self.map.contains_key(key_code) {
            self.map.insert(*key_code, vec![key]);
            return;
        }

        let key_list = self.map.get_mut(key_code).unwrap();
        if !key_list.contains(&key) {
            key_list.push(key);
        }
    }

    pub fn clear_mapping_by_key(&mut self, key: Key) {
        for (_, gameboy_key_list) in self.map.iter_mut() {
            if let Some(position) = gameboy_key_list.iter().position(|k| *k == key) {
                gameboy_key_list.remove(position);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Controls {
    pub selected_type: Type,
    pub keyboard_map: KeyboardMap,
}

impl Default for Controls {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(VirtualKeyCode::W, vec![Key::Up]);
        map.insert(VirtualKeyCode::A, vec![Key::Left]);
        map.insert(VirtualKeyCode::S, vec![Key::Down]);
        map.insert(VirtualKeyCode::D, vec![Key::Right]);
        map.insert(VirtualKeyCode::LShift, vec![Key::B]);
        map.insert(VirtualKeyCode::Space, vec![Key::A]);
        map.insert(VirtualKeyCode::Return, vec![Key::Start]);
        map.insert(VirtualKeyCode::K, vec![Key::Select]);

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

        let key_to_string_map = create_gameboy_key_to_string_map();

        let mut key_to_keycode_map: HashMap<String, Vec<&VirtualKeyCode>> = HashMap::new();

        for (keyboard_key, gameboy_key_list) in &self.map {
            for gameboy_key in gameboy_key_list {
                let gameboy_key_string = key_to_string_map.get(gameboy_key).unwrap();
                if !key_to_keycode_map.contains_key(gameboy_key_string) {
                    key_to_keycode_map.insert(gameboy_key_string.clone(), vec![keyboard_key]);
                    continue;
                }
                let keyboard_key_list = key_to_keycode_map.get_mut(gameboy_key_string).unwrap();
                keyboard_key_list.push(keyboard_key);
            }
        }

        for (gameboy_key_string, keyboard_key_list) in key_to_keycode_map {
            map.serialize_entry(&gameboy_key_string, &keyboard_key_list)
                .unwrap();
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

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Keyboard Map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keyboard_map: HashMap<VirtualKeyCode, Vec<Key>> =
            HashMap::with_capacity(map.size_hint().unwrap_or(0));
        let key_to_string_map = create_gameboy_key_to_string_map();

        while let Some((gameboy_key_string, keyboard_button_list)) =
            map.next_entry::<String, Vec<VirtualKeyCode>>()?
        {
            let gameboy_key = get_key_by_string(&key_to_string_map, gameboy_key_string);
            for keyboard_button in keyboard_button_list {
                if !keyboard_map.contains_key(&keyboard_button) {
                    keyboard_map.insert(keyboard_button, vec![gameboy_key]);
                    continue;
                }

                let gameboy_key_list = keyboard_map.get_mut(&keyboard_button).unwrap();
                gameboy_key_list.push(gameboy_key);
            }
        }

        Ok(KeyboardMap { map: keyboard_map })
    }
}

// This map is used for displaying a string as key in the config file
fn create_gameboy_key_to_string_map() -> HashMap<Key, String> {
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
