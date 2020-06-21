use std::{env, fs, process, thread};

use crate::audio_output::CpalAudioOutput;
use crate::savegame::filesystem_ram_dumper::FilesystemRamDumper;

use lib_gbemulation::apu::apu::Apu;

use lib_gbemulation::cartridge;
use lib_gbemulation::cpu::cpu::Cpu;
use lib_gbemulation::emulation::Emulation;
use lib_gbemulation::gpu::gpu::Gpu;
use lib_gbemulation::gpu::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use lib_gbemulation::io::joypad::{Joypad, Key};
use lib_gbemulation::memory::mmu::Mmu;

use crate::config::config_storage::ConfigStorage;
use crate::graphics::gameboy_screen::GameboyScreen;
use crate::graphics::gui::Gui;
use crate::graphics::window::GraphicsWindow;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;

mod audio_output;
mod config;
mod controls;
mod graphics;
mod savegame;
mod screen;

pub enum EmulationSignal {
    Cycle,
    Quit,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: gbemulator rom.gb");
        process::exit(1);
    }

    let config_storage = ConfigStorage::create_from_file("gbemulator.toml").unwrap();

    let rom_filename = String::from(&args[1]);

    let (emulation_signal_sender, emulation_signal_receiver) = channel();

    let audio_emulation_signal_sender = emulation_signal_sender.clone();

    let gameboy_screen = Arc::new(GameboyScreen::new());
    let cloned_screen = Arc::clone(&gameboy_screen);

    let window = GraphicsWindow::new(160 * 3, 144 * 3);

    //let (event_sender, event_receiver) = channel();
    let (error_sender, error_receiver) = channel();

    let rom = match fs::read(&rom_filename) {
        Ok(rom) => rom,
        Err(_) => {
            eprintln!("Could not open file {}", &rom_filename);
            process::exit(2);
        }
    };

    let (keyboard_sender, keyboard_receiver) = controls::new_keyboard(&config_storage);

    let ram_dumper = FilesystemRamDumper::new(&rom_filename);

    let emulation_thread = thread::Builder::new()
        .name("emulation".to_string())
        .spawn(move || {
            //Cpal needs to be startet from a different thread because of a winit bug on windows
            let mut audio_output = CpalAudioOutput::new(2048, Some(audio_emulation_signal_sender));

            let default_device = audio_output.get_default_device_name();
            println!("Using audio device: {}", default_device);

            audio_output.start(default_device);

            let mut joypad = Joypad::new();

            let mut cartridge = match cartridge::new_cartridge(rom, Some(Box::new(ram_dumper))) {
                Ok(cartridge) => cartridge,
                Err(message) => {
                    error_sender.send(message).unwrap();
                    return;
                }
            };

            let mut apu = Apu::new(&mut audio_output);
            let mut gpu = Gpu::new(cloned_screen);
            let mut mmu = Mmu::new(&mut *cartridge, &mut gpu, &mut apu);
            let mut cpu = Cpu::new();
            let mut emulation = Emulation::new();

            loop {
                let signal = emulation_signal_receiver.recv().unwrap();

                if let EmulationSignal::Quit = signal {
                    mmu.save();
                    break;
                }

                keyboard_receiver.receive(&mut joypad);

                emulation.cycle(&mut cpu, &mut mmu, &mut joypad);
            }
        })
        .unwrap();

    let mut gui = Gui::new(Arc::clone(&config_storage.config));

    window.start(keyboard_sender, gameboy_screen, gui);
    emulation_signal_sender.send(EmulationSignal::Quit).unwrap();
    emulation_thread.join();
}
