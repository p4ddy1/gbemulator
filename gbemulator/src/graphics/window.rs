extern crate glium;

use glium::glutin;

use self::glium::backend::glutin::glutin::GlRequest::Specific;
use self::glium::backend::glutin::glutin::{Api, GlProfile, GlRequest};
use self::glium::texture::{
    MipmapsOption, RawImage2d, UncompressedFloatFormat, UncompressedIntFormat,
    UncompressedUintFormat,
};

use crate::config::config::Config;
use crate::config::config_storage::ConfigStorage;
use crate::controls::keyboard_receiver::KeyboardReceiver;
use crate::controls::keyboard_sender::KeyboardSender;
use lib_gbemulation::gpu::{Screen, BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use crate::graphics::gameboy_screen::GameboyScreen;

pub struct GraphicsWindow {
    width: u32,
    height: u32,
}

impl GraphicsWindow {
    pub fn new(width: u32, height: u32) -> Self {
        GraphicsWindow {
            width: width,
            height: height,
        }
    }

    pub fn start(&self, keyboard_sender: KeyboardSender, gameboy_screen: Arc<GameboyScreen>) {
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

        event_loop.run(move |event, _, control_flow| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match input.state {
                            winit::event::ElementState::Pressed => {
                                keyboard_sender.press_key(keycode)
                            }
                            winit::event::ElementState::Released => {
                                keyboard_sender.release_key(keycode)
                            }
                        }
                    }
                }
                _ => {}
            },
            glutin::event::Event::MainEventsCleared => {
                let mut frame = display.draw();
                gameboy_screen.draw_to_frame(&display, &mut frame);
                frame.finish().unwrap();
            }
            _ => {}
        });
    }
}
