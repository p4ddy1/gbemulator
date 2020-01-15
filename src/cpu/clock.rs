pub struct Clock {
    machine_cycles: usize,
    clock_cycles: usize,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            machine_cycles: 0,
            clock_cycles: 0,
        }
    }

    pub fn cycle(&mut self, clock_cycles: usize) {
        self.clock_cycles += clock_cycles;
        self.machine_cycles += clock_cycles / 4;
    }
}
