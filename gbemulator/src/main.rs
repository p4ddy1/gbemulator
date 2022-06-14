#![windows_subsystem = "windows"]

use crate::config::config_storage::ConfigStorage;
use crate::graphics::gameboy_screen::{GameboyScreen, MENU_BAR_HEIGHT};

use crate::graphics::window::GraphicsWindow;

use std::sync::Arc;

mod audio_output;
mod config;
mod controls;
mod emulation;
mod graphics;
mod savegame;

pub enum EmulationSignal {
    Cycle,
    Quit,
}
pub fn main() {
    let config_storage = ConfigStorage::create_from_file("gbemulator.toml".to_string()).unwrap();
    let mut window =
        GraphicsWindow::new(160 * 3, (144 * 3) + MENU_BAR_HEIGHT as u32, &config_storage);

    let gameboy_screen = Arc::new(GameboyScreen::new(config_storage.config.clone()));
    pollster::block_on(window.start(gameboy_screen));

    config_storage.save_to_file().unwrap();
}
