# GBemulator
![Build](https://github.com/p4ddy1/gbemulator/workflows/Build/badge.svg?branch=master)

Gameboy Emulator written from scratch in Rust. Currently under heavy development.

![MarioLand2](https://cloud.lpnw.de/apps/files_sharing/publicpreview/m3FjZqCPjqY3XAj?x=2560&y=966&a=true)

Run it with a ROM as argument
```
# gbemulator rom.gb
```

### Controls
Controls are currently fixed and not configurable. You will be able to remap them soon.

| Gameboy | Keyboard   |
|---------|------------|
| A       | Spacebar   |
| B       | Left Shift |
| Up      | W          |
| Left    | A          |
| Down    | S          |
| Right   | D          |
| Start   | Return     |
| Select  | K          |

## Status

### Working
* Implemented almost all instructions (STOP still missing)
* blargg's cpu_instr and instr_timing tests pass
* Rendering is working
* Sound (Channnels 1,2,3)
* Tetris, Dr. Mario, Super Mario Land 2, Kirby's Dreamland and a lot more are working
* Timer
* Window


### Todo
* Noise Channel
* Configuration for controls, audio output, etc
* MBC
* Interrupts (Serial and Joypad)
* GUI
* Probably a lot i forgot
* Fix a few graphic glitches

## Screenshots

![CpuTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/KbyxSCrXL9kKr8i?x=1920&y=632&a=true)
![TimingTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/CE8dENP7JacDSN5?x=1920&y=632&a=true)
![Tetris](https://cloud.lpnw.de/apps/files_sharing/publicpreview/jcm8QLoHETHRFBa?x=1920&y=632&a=true)
![DrMario](https://cloud.lpnw.de/apps/files_sharing/publicpreview/MHNYnr2pPDrneGc?x=1920&y=632&a=true)
![MarioLand](https://cloud.lpnw.de/apps/files_sharing/publicpreview/freAayx9sFQk7oy?x=1920&y=632&a=true)
