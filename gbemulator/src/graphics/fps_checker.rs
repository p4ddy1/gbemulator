use std::time::{Duration, Instant};

#[allow(dead_code)]
pub struct FpsChecker {
    pub fps_bound: u16,
    pub average_frames: u16,
    frame_counter: u16,
    elapsed_time: Instant,
    frame_sum: u16,
    sample_count: u16,
    active: bool,
}

impl FpsChecker {
    #[allow(dead_code)]
    pub fn new(fps_bound: u16) -> Self {
        FpsChecker {
            frame_counter: 0,
            average_frames: 0,
            elapsed_time: Instant::now(),
            frame_sum: 0,
            sample_count: 0,
            active: true,
            fps_bound,
        }
    }

    #[allow(dead_code)]
    pub fn count_frame(&mut self) {
        if !self.active {
            return;
        }

        self.frame_counter += 1;

        if self.elapsed_time.elapsed() >= Duration::from_secs(1) {
            self.frame_sum += self.frame_counter;
            self.sample_count += 1;
            self.frame_counter = 0;
            self.elapsed_time = Instant::now();

            if self.sample_count > 1 {
                self.average_frames = self.frame_sum / self.sample_count;
                self.frame_sum = 0;
                self.sample_count = 0;

                self.active = false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn should_limit_frames(&self) -> bool {
        if self.active {
            return false;
        }

        if self.average_frames > self.fps_bound {
            return true;
        }

        false
    }
}
