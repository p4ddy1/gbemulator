pub struct Clock {
    pub cpu_clock_hz: usize,
    pub clock_cycles_passed_frame: usize,
    pub machine_cycles_passed_frame: usize,
    pub clock_cycles_per_frame: usize,
    pub frame_time_s: f32,
}

impl Clock {
    pub fn new(cpu_clock_hz: usize, fps: f32) -> Clock {
        let clock_cycles_per_frame: usize = (cpu_clock_hz as f32 / fps) as usize;
        let frame_time: f32 = 1.0 / fps;

        Clock {
            cpu_clock_hz: cpu_clock_hz,
            clock_cycles_passed_frame: 0,
            machine_cycles_passed_frame: 0,
            clock_cycles_per_frame: clock_cycles_per_frame,
            frame_time_s: frame_time,
        }
    }

    pub fn cycle(&mut self, clock_cycles: u8) {
        self.clock_cycles_passed_frame += clock_cycles as usize;
        self.machine_cycles_passed_frame += (clock_cycles / 4) as usize;
    }

    pub fn reset(&mut self) {
        let cycles_passed = self.clock_cycles_passed_frame;
        self.clock_cycles_passed_frame = cycles_passed - self.clock_cycles_per_frame;
        self.machine_cycles_passed_frame = (cycles_passed - self.clock_cycles_per_frame) / 4;
    }
}
