use std::{env, process};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

use lib_gbemulation::cartridge::mbc1_cartridge::Mbc1Cartridge;
use lib_gbemulation::cpu::cpu::Cpu;
use lib_gbemulation::emulation::Emulation;
use lib_gbemulation::gpu::gpu::Gpu;
use crate::screen::SdlScreen;
use lib_gbemulation::gpu::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use lib_gbemulation::io::joypad::{Joypad, Key};
use lib_gbemulation::memory::mmu::Mmu;

mod screen;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: gbemulator rom.gb");
        process::exit(1);
    }

    let rom_filename = String::from(&args[1]);

    let mut cartridge = match Mbc1Cartridge::new_from_file(rom_filename) {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

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

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();
    let mut screen = SdlScreen::new(
        canvas,
        texture,
        SCREEN_WIDTH as u16 * SCALE as u16,
        SCREEN_HEIGHT as u16 * SCALE as u16,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut joypad = Joypad::new();

    let mut gpu = Gpu::new(&mut screen);
    let mut mmu = Mmu::new(&mut cartridge, &mut gpu);
    let mut cpu = Cpu::new();
    let mut emulation = Emulation::new();

    loop {
        if !handle_sdl_events(&mut event_pump, &mut joypad) {
            process::exit(0);
        }
        emulation.cycle(&mut cpu, &mut mmu, &mut joypad);
    }
}

fn handle_sdl_events(event_pump: &mut EventPump, joypad: &mut Joypad) -> bool {
    for event in event_pump.poll_iter() {
        match event {
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
            Event::Quit { .. } => {
                return false;
            }
            _ => {}
        }
    }

    true
}
