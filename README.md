# GBemulator
![Build](https://github.com/p4ddy1/gbemulator/workflows/Build/badge.svg?branch=master)

Gameboy Emulator written from scratch in Rust. Compatible with macOS, Linux and Windows. 
Uses [wgpu](https://github.com/gfx-rs/wgpu) for graphics, [cpal](https://github.com/RustAudio/cpal) for audio output 
and [egui](https://github.com/emilk/egui) for the interface. 
Binaries for Linux and Windows are available in the release section.
If you want to compile it from source just run `cargo build --release` or `cargo run --release`

![MarioLand2](https://cloud.lpnw.de/apps/files_sharing/publicpreview/Ee5piRQ624cn84c?x=2549&y=980&a=true)
![MarioLand](https://cloud.lpnw.de/apps/files_sharing/publicpreview/cQCjwKrGMwYi7b8?x=2549&y=980&a=true)

## Status

### Working
* Implemented almost all instructions (STOP still missing)
* blargg's cpu_instr and instr_timing tests pass
* Rendering is working
* Sound
* Tetris, Dr. Mario, Super Mario Land 2, Kirby's Dreamland and a lot more are working
* Timer
* Window
* GUI
* Configurable controls
* Configurable palette


### Todo
* Complete APU
* MBC
* Interrupts (Serial and Joypad)
* Probably a lot I forgot

## Screenshots

![CpuTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/KbyxSCrXL9kKr8i?x=1920&y=632&a=true)
![TimingTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/CE8dENP7JacDSN5?x=1920&y=632&a=true)
