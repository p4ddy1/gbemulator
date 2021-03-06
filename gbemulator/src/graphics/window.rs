extern crate glium;

use glium::glutin;

use self::glium::backend::glutin::glutin::Api;
use self::glium::backend::glutin::glutin::GlRequest::Specific;

use crate::graphics::gameboy_screen::GameboyScreen;
use crate::graphics::gui::Gui;

use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::sync::{Arc, Mutex};

use crate::config::config_storage::ConfigStorage;

use crate::controls::keyboard_controller::KeyboardController;
use crate::emulation::Emulation;
use crate::EmulationSignal;
use lib_gbemulation::io::joypad::Joypad;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use winit::event::KeyboardInput;
use winit::platform::desktop::EventLoopExtDesktop;

pub struct GraphicsWindow<'a> {
    width: u32,
    height: u32,
    config_storage: &'a ConfigStorage,
    emulation_signal_sender: Option<Rc<Sender<EmulationSignal>>>,
}

impl<'a> GraphicsWindow<'a> {
    pub fn new(width: u32, height: u32, config_storage: &'a ConfigStorage) -> Self {
        GraphicsWindow {
            width,
            height,
            config_storage,
            emulation_signal_sender: None,
        }
    }

    pub fn start(&mut self, gameboy_screen: Arc<GameboyScreen>) {
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

        let joypad = Arc::new(Mutex::new(Joypad::new()));

        let emulation = Emulation::new(Arc::clone(&gameboy_screen), Arc::clone(&joypad));

        let keyboard_controller = KeyboardController::new(joypad, &self.config_storage);

        let (rom_filename_sender, rom_filename_receiver) = channel();

        let mut gui = Gui::new(Arc::clone(&self.config_storage.config), rom_filename_sender);

        event_loop.run_return(move |event, _, control_flow| {
            //Imgui also needs to handle events
            platform.handle_event(imgui.io_mut(), display.gl_window().window(), &event);

            self.start_emulation(&rom_filename_receiver, &emulation);

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        gui.set_keyboard_input(input);
                        handle_inputs(&keyboard_controller, &input)
                    }
                    _ => {}
                },
                glutin::event::Event::MainEventsCleared => {
                    let dimensions = display.get_framebuffer_dimensions();
                    let gl_window = display.gl_window();
                    platform
                        .prepare_frame(imgui.io_mut(), &gl_window.window())
                        .unwrap();

                    let mut ui = imgui.frame();

                    gui.render(&mut ui);

                    let mut target = display.draw();

                    gameboy_screen.draw_to_frame(&display, &mut target, dimensions.0, dimensions.1);

                    platform.prepare_render(&ui, gl_window.window());
                    let draw_data = ui.render();
                    renderer.render(&mut target, draw_data).unwrap();

                    target.finish().unwrap();
                }
                _ => {}
            }
        });
    }

    fn start_emulation(
        &mut self,
        rom_filename_receiver: &Receiver<Option<String>>,
        emulation: &Emulation,
    ) {
        if let Ok(filename) = rom_filename_receiver.try_recv() {
            let rom_file: String;
            match filename {
                Some(file) => rom_file = file,
                None => {
                    return;
                }
            }
            if let Some(sender) = &self.emulation_signal_sender {
                //Stop running emulation
                sender.send(EmulationSignal::Quit).unwrap();
            }

            let sender = emulation.start(&rom_file).unwrap();

            self.emulation_signal_sender = Some(Rc::new(sender));
        }
    }
}

fn handle_inputs(keyboard_controller: &KeyboardController, input: &KeyboardInput) {
    if let Some(keycode) = input.virtual_keycode {
        match input.state {
            winit::event::ElementState::Pressed => keyboard_controller.push_key(keycode),
            winit::event::ElementState::Released => keyboard_controller.release_key(keycode),
        }
    }
}
