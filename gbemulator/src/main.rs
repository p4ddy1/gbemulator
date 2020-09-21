use std::{env, fs, process, thread};

use crate::audio_output::CpalAudioOutput;
use crate::savegame::filesystem_ram_dumper::FilesystemRamDumper;

use lib_gbemulation::apu::apu::Apu;

use lib_gbemulation::cartridge;
use lib_gbemulation::cpu::cpu::Cpu;
use lib_gbemulation::emulation::Emulation;
use lib_gbemulation::gpu::gpu::Gpu;

use lib_gbemulation::io::joypad::Joypad;
use lib_gbemulation::memory::mmu::Mmu;

use crate::config::config_storage::ConfigStorage;
use crate::graphics::gameboy_screen::{GameboyScreen, MENU_BAR_HEIGHT};
use crate::graphics::gui::Gui;
use crate::graphics::window::GraphicsWindow;
use std::sync::mpsc::channel;
use std::sync::Arc;

mod audio_output;
mod config;
mod controls;
mod emulation;
mod graphics;
mod savegame;
mod screen;

pub enum EmulationSignal {
    Cycle,
    Quit,
}

fn main() {
    let config_storage = ConfigStorage::create_from_file("gbemulator.toml".to_string()).unwrap();
    let window = GraphicsWindow::new(160 * 3, (144 * 3) + MENU_BAR_HEIGHT as u32, &config_storage);

    let gameboy_screen = Arc::new(GameboyScreen::new());
    window.start(gameboy_screen);

    config_storage.save_to_file();
}
