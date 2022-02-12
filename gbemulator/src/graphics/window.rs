use crate::graphics::gameboy_screen::GameboyScreen;

use std::sync::{Arc, Mutex};

use crate::config::config_storage::ConfigStorage;

use crate::controls::keyboard_controller::KeyboardController;
use crate::emulation::Emulation;
use crate::graphics::fps_checker::FpsChecker;
use crate::graphics::gui::emulator_app::EmulatorApp;
use crate::EmulationSignal;
use egui::FontDefinitions;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::PlatformDescriptor;
use epi::App;
use lib_gbemulation::gpu::{SCREEN_HEIGHT, SCREEN_WIDTH};
use lib_gbemulation::io::joypad::Joypad;
use std::rc::Rc;
use std::string::String;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use wgpu::{FilterMode, Surface};
use winit::dpi::PhysicalSize;
use winit::event::KeyboardInput;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct GraphicsWindow<'a> {
    width: u32,
    height: u32,
    config_storage: &'a ConfigStorage,
    emulation_signal_sender: Option<Rc<Sender<EmulationSignal>>>,
}

struct ExampleRepaintSignal;

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {}
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

    pub async fn start(&mut self, gameboy_screen: Arc<GameboyScreen>) {
        let mut event_loop = EventLoop::new();

        let size = winit::dpi::PhysicalSize {
            width: self.width,
            height: self.height,
        };

        let window = WindowBuilder::new()
            .with_title("GBemulator")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();

        let wgpu_instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { wgpu_instance.create_surface(&window) };

        let adapter = wgpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let texture_size = wgpu::Extent3d {
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
            depth_or_array_layers: 1,
        };

        let screen_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Screen Texture"),
        });

        let joypad = Arc::new(Mutex::new(Joypad::new()));

        let emulation = Emulation::new(Arc::clone(&gameboy_screen), Arc::clone(&joypad));

        let keyboard_controller = KeyboardController::new(joypad, &self.config_storage);

        let (rom_filename_sender, rom_filename_receiver) = channel();

        let mut platform = egui_winit_platform::Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: std::default::Default::default(),
        });

        let mut egui_rpass = egui_wgpu_backend::RenderPass::new(&device, config.format, 1);

        let mut emulator_gui_app =
            EmulatorApp::new(rom_filename_sender, &self.config_storage.config);

        let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal {});

        let mut fps_checker = FpsChecker::new(240);

        let mut limit_msg_shown = false;

        event_loop.run_return(move |event, _, control_flow| {
            platform.handle_event(&event);
            self.start_emulation(&rom_filename_receiver, &emulation);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        println!("Closing...");
                        return;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        emulator_gui_app.set_keyboard_input(input);
                        handle_inputs(&keyboard_controller, &input);
                    }
                    WindowEvent::Resized(physical_size) => {
                        resize(&surface, &mut config, &device, physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        resize(&surface, &mut config, &device, *new_inner_size);
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    let output = surface.get_current_texture().unwrap();
                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                    platform.begin_frame();
                    let app_output = epi::backend::AppOutput::default();
                    let mut frame = epi::Frame::new(epi::backend::FrameData {
                        info: epi::IntegrationInfo {
                            name: "emulator_egui",
                            web_info: None,
                            cpu_usage: None,
                            native_pixels_per_point: Some(window.scale_factor() as _),
                            prefer_dark_mode: None,
                        },
                        output: app_output,
                        repaint_signal: repaint_signal.clone(),
                    });

                    emulator_gui_app.set_tex(egui_rpass.egui_texture_from_wgpu_texture(
                        &device,
                        &screen_texture,
                        FilterMode::Nearest,
                    ));

                    emulator_gui_app.update(&platform.context(), &mut frame);

                    let (_output, paint_commands) = platform.end_frame(Some(&window));
                    let paint_jobs = platform.context().tessellate(paint_commands);

                    let screen_descriptor = ScreenDescriptor {
                        physical_width: config.width,
                        physical_height: config.height,
                        scale_factor: window.scale_factor() as f32,
                    };

                    egui_rpass.update_texture(&device, &queue, &platform.context().font_image());
                    egui_rpass.update_user_textures(&device, &queue);
                    egui_rpass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

                    egui_rpass
                        .execute(
                            &mut encoder,
                            &view,
                            &paint_jobs,
                            &screen_descriptor,
                            Some(wgpu::Color::BLACK),
                        )
                        .unwrap();

                    gameboy_screen.draw_to_queue(&queue, &screen_texture, texture_size);

                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();

                    fps_checker.count_frame();

                    if fps_checker.should_limit_frames() {
                        if !limit_msg_shown {
                            println!(
                                "Running with: {} FPS. Limiting to 60",
                                fps_checker.average_frames
                            );
                            limit_msg_shown = true;
                        }
                        let dur = Duration::from_secs_f64(1.0 / 60.0);
                        sleep(dur);
                    }
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

fn resize(
    surface: &Surface,
    config: &mut wgpu::SurfaceConfiguration,
    device: &wgpu::Device,
    new_size: PhysicalSize<u32>,
) {
    config.width = new_size.width;
    config.height = new_size.height;
    surface.configure(device, config);
}
