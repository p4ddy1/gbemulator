use std::{env, fs, process, thread};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use crate::audio_output::CpalAudioOutput;
use crate::savegame::filesystem_ram_dumper::FilesystemRamDumper;
use crate::screen::{ScreenBuffer, SdlScreen};

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

mod audio_output;
mod fps_checker;
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

    let rom_filename = String::from(&args[1]);

    let (emulation_signal_sender, emulation_signal_receiver) = channel();

    let audio_emulation_signal_sender = emulation_signal_sender.clone();
    let mut audio_output = CpalAudioOutput::new(44100, 2048, Some(audio_emulation_signal_sender));
    audio_output.start();

    //TODO: This SDL stuff is just for testing purposes. In the future a better method is needed with some GUI stuff
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Gameboy Emulator",
            SCREEN_WIDTH as u32 * SCALE as u32,
            SCREEN_HEIGHT as u32 * SCALE as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();

    let screen_buffer = Arc::new(ScreenBuffer::new());

    let mut screen = SdlScreen::new(
        canvas,
        texture,
        SCREEN_WIDTH as u16 * SCALE as u16,
        SCREEN_HEIGHT as u16 * SCALE as u16,
        &screen_buffer,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    let (event_sender, event_receiver) = channel();
    let (error_sender, error_receiver) = channel();

    let cloned_screen_buffer = Arc::clone(&screen_buffer);

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
            let mut joypad = Joypad::new();

            let mut cartridge = match cartridge::new_cartridge(rom, Some(Box::new(ram_dumper))) {
                Ok(cartridge) => cartridge,
                Err(message) => {
                    error_sender.send(message).unwrap();
                    return;
                }
            };

            let mut apu = Apu::new(&mut audio_output);
            let mut gpu = Gpu::new(cloned_screen_buffer);
            let mut mmu = Mmu::new(&mut *cartridge, &mut gpu, &mut apu);
            let mut cpu = Cpu::new();
            let mut emulation = Emulation::new();

            loop {
                let signal = emulation_signal_receiver.recv().unwrap();

                if let EmulationSignal::Quit = signal {
                    mmu.save();
                    break;
                }

                match event_receiver.try_recv() {
                    //TODO: Make this configurable
                    Ok(event) => match event {
                        Event::KeyDown {
                            keycode: Some(Keycode::Right),
                            ..
                        } => {
                            joypad.push_key(Key::Right);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::Right),
                            ..
                        } => {
                            joypad.release_key(Key::Right);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Left),
                            ..
                        } => {
                            joypad.push_key(Key::Left);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::Left),
                            ..
                        } => {
                            joypad.release_key(Key::Left);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Down),
                            ..
                        } => {
                            joypad.push_key(Key::Down);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::Down),
                            ..
                        } => {
                            joypad.release_key(Key::Down);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Up),
                            ..
                        } => {
                            joypad.push_key(Key::Up);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::Up),
                            ..
                        } => {
                            joypad.release_key(Key::Up);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Space),
                            ..
                        } => {
                            joypad.push_key(Key::Start);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::Space),
                            ..
                        } => {
                            joypad.release_key(Key::Start);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::B),
                            ..
                        } => {
                            joypad.push_key(Key::B);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::B),
                            ..
                        } => {
                            joypad.release_key(Key::B);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::A),
                            ..
                        } => {
                            joypad.push_key(Key::A);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::A),
                            ..
                        } => {
                            joypad.release_key(Key::A);
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::E),
                            ..
                        } => {
                            joypad.push_key(Key::Select);
                        }
                        Event::KeyUp {
                            keycode: Some(Keycode::E),
                            ..
                        } => {
                            joypad.release_key(Key::Select);
                        }
                        _ => {}
                    },
                    _ => {}
                }

                emulation.cycle(&mut cpu, &mut mmu, &mut joypad);
            }
        })
        .unwrap();

    let mut fps_checker = FpsChecker::new(300);

    'video: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    emulation_signal_sender.send(EmulationSignal::Quit).unwrap();
                    break 'video;
                }
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    event_sender.send(event).unwrap();
                }
                _ => {}
            }
        }

        if let Ok(error_message) = error_receiver.try_recv() {
            eprintln!("{}", error_message);
            break;
        }

        fps_checker.count_frame();

        screen.present();

        if fps_checker.should_limit_frames() {
            thread::sleep(Duration::from_secs_f32(1.0 / 60.0));
        }
    }

    println!("Bye");
}
