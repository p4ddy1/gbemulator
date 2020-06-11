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

use crate::fps_checker::FpsChecker;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use crate::graphics_window::GraphicsWindow;

mod audio_output;
mod fps_checker;
mod savegame;
mod screen;
mod graphics_window;

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

    let rom_filename = String::from(&args[1]);

    let (emulation_signal_sender, emulation_signal_receiver) = channel();

    let audio_emulation_signal_sender = emulation_signal_sender.clone();
    let window = Arc::new(GraphicsWindow::new(160,144));

    let wind = Arc::clone(&window);

    //let (event_sender, event_receiver) = channel();
    let (error_sender, error_receiver) = channel();

    let rom = match fs::read(&rom_filename) {
        Ok(rom) => rom,
        Err(_) => {
            eprintln!("Could not open file {}", &rom_filename);
            process::exit(2);
        }
    };

    let ram_dumper = FilesystemRamDumper::new(&rom_filename);

    thread::Builder::new()
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
            let mut gpu = Gpu::new(wind);
            let mut mmu = Mmu::new(&mut *cartridge, &mut gpu, &mut apu);
            let mut cpu = Cpu::new();
            let mut emulation = Emulation::new();

            loop {
                let signal = emulation_signal_receiver.recv().unwrap();

                if let EmulationSignal::Quit = signal {
                    mmu.save();
                    break;
                }

                emulation.cycle(&mut cpu, &mut mmu, &mut joypad);
            }
        })
        .unwrap();

    let mut fps_checker = FpsChecker::new(300);

    window.start();

    /* loop {
         thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
     }*/

    /*'video: loop {
        if let Ok(error_message) = error_receiver.try_recv() {
            eprintln!("{}", error_message);
            break;
        }


        fps_checker.count_frame();

        if fps_checker.should_limit_frames() {
            thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
        }
    }*/

    println!("Bye");
}
