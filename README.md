# GBemulator
![BuildStatus](https://api.travis-ci.org/p4ddy1/gbemulator.svg?branch=master)

Gameboy Emulator written from scratch in Rust. This is currently nowhere near a fully functional emulator. 
I'm developing this just for fun and educational purposes. Come back a little later ;)

![MarioLand](https://cloud.lpnw.de/apps/files_sharing/publicpreview/freAayx9sFQk7oy?x=1920&y=632&a=true)

## Status

### Working
* Implemented almost all instructions (HALT and STOP still missing)
* blargg's cpu_instr and instr_timing tests pass
* Basic Background rendering is working
* Basic Sprite rendering is working
* Tetris works
* Super Mario Land is playable
* Timer

### Todo
* Switching the background set
* Window
* Some Sprite options
* MBC
* Interrupts (Serial and Joypad)
* Sound
* GUI
* Probably a lot i forgot
* Fix a few graphic glitches

## Screenshots

![CpuTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/KbyxSCrXL9kKr8i?x=1920&y=632&a=true)
![TimingTest](https://cloud.lpnw.de/apps/files_sharing/publicpreview/CE8dENP7JacDSN5?x=1920&y=632&a=true)
![Tetris](https://cloud.lpnw.de/apps/files_sharing/publicpreview/jcm8QLoHETHRFBa?x=1920&y=632&a=true)
![DrMario](https://cloud.lpnw.de/apps/files_sharing/publicpreview/MHNYnr2pPDrneGc?x=1920&y=632&a=true)
