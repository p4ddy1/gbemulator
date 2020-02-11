use crate::cartridge::mbc1_cartridge::Mbc1Cartridge;
use crate::cartridge::small_cartridge::SmallCartridge;
use crate::cpu::cpu::Cpu;
use crate::gpu::gpu::Gpu;
use crate::gpu::screen::SdlScreen;
use crate::gpu::{SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::io::joypad::{Joypad, Key};
use crate::memory::mmu::Mmu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::thread;
use std::time::Duration;

mod cartridge;
mod cpu;
mod gpu;
mod io;
mod memory;
mod util;

fn main() {
    let mut cartridge = match Mbc1Cartridge::new_from_file("testrom/marioland.gb") {
        Ok(c) => c,
        Err(e) => {
            panic!(e);
        }
    };

    let bios = match SmallCartridge::new_from_file("testrom/bios.gb") {
        Ok(c) => c,
        Err(e) => {
            panic!(e);
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
    let mut texture = texture_creator
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
    let mut mmu = Mmu::new(&mut cartridge, &mut gpu, Some(&bios), &mut joypad);
    let mut cpu = Cpu::new(&mut mmu);

    const CPU_CLOCK_HZ: usize = 4194304;
    const FPS: usize = 60;
    const CLOCK_CYCLES_PER_FRAME: usize = CPU_CLOCK_HZ / FPS;
    const FRAME_TIME_NS: u64 = 1000000000 / FPS as u64;

    const DIV_DIVIDER: usize = 256;

    let mut clock_cycles_passed_frame = 0;
    let mut clock_cycles_passed_timer = 0;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Right);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Left);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Down);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Up);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Start);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Start);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::B);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::B),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::B);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::A);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::A);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    cpu.mmu.joypad.push_key(Key::Select);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    cpu.mmu.joypad.release_key(Key::Select);
                }
                Event::Quit { .. } => {
                    break 'mainloop;
                }
                _ => {}
            }
        }

        //TODO: Check if this is the correct way
        while clock_cycles_passed_frame < CLOCK_CYCLES_PER_FRAME {
            let last_cycle = cpu.execute_program_counter();
            clock_cycles_passed_frame += last_cycle as usize;

            clock_cycles_passed_timer += clock_cycles_passed_frame;

            if clock_cycles_passed_timer % DIV_DIVIDER == 0 {
                cpu.mmu.increase_divider();
            }

            if clock_cycles_passed_timer > CPU_CLOCK_HZ {
                clock_cycles_passed_timer = 0;
            }
        }

        clock_cycles_passed_frame = 0;

        cpu.mmu.gpu.screen.present();

        thread::sleep(Duration::from_nanos(FRAME_TIME_NS));
    }
}
