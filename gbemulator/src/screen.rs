/*
pub struct ScreenBuffer {
    pub buffer1: Mutex<[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]>,
    pub buffer2: Mutex<[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]>,
    pub current_buffer: Mutex<u8>,
}

impl ScreenBuffer {
    pub fn new() -> Self {
        ScreenBuffer {
            buffer1: Mutex::new([255; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]),
            buffer2: Mutex::new([255; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]),
            current_buffer: Mutex::new(1),
        }
    }
}

impl Screen for ScreenBuffer {
    fn draw(&self, screen_buffer: &[u8; (SCREEN_WIDTH * SCREEN_WIDTH * 3) + SCREEN_HEIGHT * 3]) {
        let mut current_buffer = self.current_buffer.lock().unwrap();

        let mut buffer = if *current_buffer == 1 {
            self.buffer1.lock().unwrap()
        } else {
            self.buffer2.lock().unwrap()
        };

        *buffer = *screen_buffer;

        if *current_buffer == 1 {
            *current_buffer = 2;
        } else {
            *current_buffer = 1;
        }
    }
}
*/
