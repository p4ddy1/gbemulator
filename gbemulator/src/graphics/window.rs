extern crate glium;

use glium::glutin;

use self::glium::backend::glutin::glutin::Api;
use self::glium::backend::glutin::glutin::GlRequest::Specific;

use crate::controls::keyboard_sender::KeyboardSender;
use crate::graphics::gameboy_screen::GameboyScreen;
use crate::graphics::gui::Gui;

use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::sync::Arc;

use winit::event::KeyboardInput;
use winit::platform::desktop::EventLoopExtDesktop;

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

    pub fn start(
        &self,
        keyboard_sender: KeyboardSender,
        gameboy_screen: Arc<GameboyScreen>,
        mut gui: Gui,
    ) {
        let mut event_loop = glutin::event_loop::EventLoop::new();

        let size: glutin::dpi::LogicalSize<u32> = (self.width, self.height).into();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("GBemulator")
            .with_inner_size(size);
        let context_builder = glutin::ContextBuilder::new()
            //We dont need the latest version
            .with_gl(Specific(Api::OpenGl, (3, 1)))
            .with_vsync(true);

        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &display.gl_window().window(),
            HiDpiMode::Rounded,
        );

        let mut renderer = Renderer::init(&mut imgui, &display).unwrap();

        event_loop.run_return(move |event, _, control_flow| {
            //Imgui also needs to handle events
            platform.handle_event(imgui.io_mut(), display.gl_window().window(), &event);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        gui.set_keyboard_input(input);
                        handle_inputs(&keyboard_sender, &input)
                    }
                    _ => {}
                },
                glutin::event::Event::MainEventsCleared => {
                    let gl_window = display.gl_window();
                    platform
                        .prepare_frame(imgui.io_mut(), &gl_window.window())
                        .unwrap();

                    let mut ui = imgui.frame();

                    gui.render(&mut ui);

                    let mut target = display.draw();

                    gameboy_screen.draw_to_frame(&display, &mut target);

                    platform.prepare_render(&ui, gl_window.window());
                    let draw_data = ui.render();
                    renderer.render(&mut target, draw_data).unwrap();

                    target.finish().unwrap();
                }
                _ => {}
            }
        });
    }
}

fn handle_inputs(keyboard_sender: &KeyboardSender, input: &KeyboardInput) {
    if let Some(keycode) = input.virtual_keycode {
        match input.state {
            winit::event::ElementState::Pressed => keyboard_sender.press_key(keycode),
            winit::event::ElementState::Released => keyboard_sender.release_key(keycode),
        }
    }
}
