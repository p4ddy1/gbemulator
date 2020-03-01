pub struct Clock {
    pub cpu_clock_hz: usize,
    pub clock_cycles_passed_frame: usize,
    pub machine_cycles_passed_frame: usize,
    pub clock_cycles_per_frame: usize,
    pub frame_time_ns: u64,
}

impl Clock {
    pub fn new(cpu_clock_hz: usize, fps: usize) -> Clock {
        let clock_cycles_per_frame: usize = cpu_clock_hz / fps;
        let frame_time: u64 = 1000000000 / fps as u64;

        Clock {
            cpu_clock_hz: cpu_clock_hz,
            clock_cycles_passed_frame: 0,
            machine_cycles_passed_frame: 0,
            clock_cycles_per_frame: clock_cycles_per_frame,
            frame_time_ns: frame_time,
        }
    }

    pub fn cycle(&mut self, clock_cycles: u8) {
        self.clock_cycles_passed_frame += clock_cycles as usize;
        self.machine_cycles_passed_frame += (clock_cycles / 4) as usize;
    }

    pub fn reset(&mut self) {
        self.clock_cycles_passed_frame = 0;
        self.machine_cycles_passed_frame = 0;
    }
}
