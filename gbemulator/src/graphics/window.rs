extern crate glium;

use glium::glutin;

use self::glium::backend::glutin::glutin::GlRequest::Specific;
use self::glium::backend::glutin::glutin::{Api, GlProfile, GlRequest};
use self::glium::texture::{
    MipmapsOption, RawImage2d, UncompressedFloatFormat, UncompressedIntFormat,
    UncompressedUintFormat,
};
use self::glium::{Rect, Surface};
use lib_gbemulation::gpu::{Screen, BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

pub struct GraphicsWindow {
    width: u32,
    height: u32,
    buffer1: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    buffer2: Arc<Mutex<[u8; BUFFER_SIZE]>>,
    current_buffer: Arc<AtomicU8>,
}

impl GraphicsWindow {
    pub fn new(width: u32, height: u32) -> Self {
        GraphicsWindow {
            width: width,
            height: height,
            buffer1: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            buffer2: Arc::new(Mutex::new([255; BUFFER_SIZE])),
            current_buffer: Arc::new(AtomicU8::new(1)),
        }
    }

    pub fn start(&self) {
        let event_loop = glutin::event_loop::EventLoop::new();

        let size: glutin::dpi::LogicalSize<u32> = (self.width, self.height).into();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("GBemulator")
            .with_inner_size(size);
        let context_builder = glutin::ContextBuilder::new()
            //We dont need the latest version
            .with_gl(Specific(Api::OpenGl, (3, 1)))
            .with_vsync(true);

        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let cloned_buffer1 = Arc::clone(&self.buffer1);
        let cloned_buffer2 = Arc::clone(&self.buffer2);
        let cloned_current_buffer = Arc::clone(&self.current_buffer);

        event_loop.run(move |event, foo, control_flow| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    println!("{:?}", input.virtual_keycode.unwrap());
                }
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {
                let mut target = display.draw();

                let current_buffer = cloned_current_buffer.load(Ordering::SeqCst);

                let data = *if current_buffer == 1 {
                    cloned_buffer1.lock().unwrap()
                } else {
                    cloned_buffer2.lock().unwrap()
                };

                let screen = RawImage2d::from_raw_rgb_reversed(
                    &data,
                    (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
                );

                let texture = glium::texture::Texture2d::with_format(
                    &display,
                    screen,
                    UncompressedFloatFormat::U8U8U8,
                    MipmapsOption::NoMipmap,
                )
                .unwrap();

                texture
                    .as_surface()
                    .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
                target.finish().unwrap();
            }
            _ => {}
        });
    }
}

impl Screen for GraphicsWindow {
    fn draw(&self, screen_buffer: &[u8; BUFFER_SIZE]) {
        let mut buffer = if self.current_buffer.load(Ordering::SeqCst) == 1 {
            self.current_buffer.store(2, Ordering::SeqCst);
            self.buffer2.lock().unwrap()
        } else {
            self.current_buffer.store(1, Ordering::SeqCst);
            self.buffer1.lock().unwrap()
        };

        *buffer = *screen_buffer;
    }
}
