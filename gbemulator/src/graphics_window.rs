extern crate glium;
use glium::{glutin};


use std::sync::{Mutex, Arc};
use lib_gbemulation::gpu::{BUFFER_SIZE, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use self::glium::{Surface, Rect};
use self::glium::texture::{UncompressedFloatFormat, UncompressedUintFormat, MipmapsOption, RawImage2d, UncompressedIntFormat};
use self::glium::backend::glutin::glutin::{GlProfile, GlRequest, Api};
use self::glium::backend::glutin::glutin::GlRequest::Specific;

pub struct GraphicsWindow {
    width: u32,
    height: u32,
    buffer: Arc<Mutex<[u8; BUFFER_SIZE]>>
}

impl GraphicsWindow {
    pub fn new(width: u32, height: u32) -> Self {
        GraphicsWindow {
            width: width,
            height: height,
            buffer: Arc::new(Mutex::new([255; BUFFER_SIZE]))
        }
    }

    pub fn start(&self) {
        let event_loop = glutin::event_loop::EventLoop::new();

        let size : glutin::dpi::LogicalSize<u32> = (self.width, self.height).into();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("GBemulator")
            .with_inner_size(size);
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(Specific(Api::OpenGl, (3, 1))) //We dont need the latest version
            .with_vsync(true);

        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let size = (self.width, self.height);

        let cloned_buffer = Arc::clone(&self.buffer);

        event_loop.run(move |event, foo, control_flow| {
            match event {
               glutin::event::Event::WindowEvent { event,.. } => match event {
                   glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => {}
                },
                glutin::event::Event::MainEventsCleared => {
                    let mut target = display.draw();
                    let data = *cloned_buffer.lock().unwrap();

                    let screen = RawImage2d::from_raw_rgb_reversed(&data, size);
                    let texture = glium::texture::Texture2d::with_format(
                        &display,
                        screen,
                        UncompressedFloatFormat::U8U8U8,
                        MipmapsOption::NoMipmap,
                    ).unwrap();

                    texture.as_surface().fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
                    target.finish().unwrap();
                }
                _ => {}
            }
        });

    }
}

impl Screen for GraphicsWindow {
    fn draw(&self, screen_buffer: &[u8; BUFFER_SIZE]) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = *screen_buffer;
    }
}