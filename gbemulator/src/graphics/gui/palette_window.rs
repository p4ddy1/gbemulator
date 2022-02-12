use crate::config::color_palette::ColorPalette;
use crate::config::config::Config;
use crate::graphics::gui::State;
use std::sync::{Arc, RwLock};

pub struct PaletteWindow {
    config: Arc<RwLock<Config>>,
    color1: [u8; 3],
    color2: [u8; 3],
    color3: [u8; 3],
    color4: [u8; 3],
}

impl PaletteWindow {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let cloned_config = Arc::clone(&config);
        let palette = &cloned_config.read().unwrap().color_palette;
        PaletteWindow {
            config,
            color1: palette.color1,
            color2: palette.color2,
            color3: palette.color3,
            color4: palette.color4,
        }
    }

    pub fn update(&mut self, ctx: &egui::CtxRef, state: &mut State) {
        egui::Window::new("Palette")
            .open(&mut state.palette_window_shown)
            .show(ctx, |ui| {
                ui.columns(2, |ui| {
                    let col1 = ui.get_mut(0).unwrap();

                    col1.label("Pick colors");

                    if col1.color_edit_button_srgb(&mut self.color1).changed() {
                        self.config.write().unwrap().color_palette.color1 = self.color1;
                    }

                    if col1.color_edit_button_srgb(&mut self.color2).changed() {
                        self.config.write().unwrap().color_palette.color2 = self.color2;
                    }

                    if col1.color_edit_button_srgb(&mut self.color3).changed() {
                        self.config.write().unwrap().color_palette.color3 = self.color3;
                    }

                    if col1.color_edit_button_srgb(&mut self.color4).changed() {
                        self.config.write().unwrap().color_palette.color4 = self.color4;
                    }

                    let col2 = ui.get_mut(1).unwrap();

                    col2.label("Presets");

                    if col2.button("Default").clicked() {
                        self.load_color_preset([
                            ColorPalette::default().color1,
                            ColorPalette::default().color2,
                            ColorPalette::default().color3,
                            ColorPalette::default().color4,
                        ]);
                    }

                    if col2.button("Metallic").clicked() {
                        self.load_color_preset([
                            [34, 30, 49],
                            [65, 72, 93],
                            [119, 142, 152],
                            [197, 219, 212],
                        ]);
                    }

                    if col2.button("Ice Cream").clicked() {
                        self.load_color_preset([
                            [124, 63, 88],
                            [235, 107, 111],
                            [249, 168, 117],
                            [255, 246, 211],
                        ]);
                    }
                });
            });
    }

    fn load_color_preset(&mut self, palette: [[u8; 3]; 4]) {
        let mut config = self.config.write().unwrap();
        self.color1 = palette[0];
        config.color_palette.color1 = self.color1;

        self.color2 = palette[1];
        config.color_palette.color2 = self.color2;

        self.color3 = palette[2];
        config.color_palette.color3 = self.color3;

        self.color4 = palette[3];
        config.color_palette.color4 = self.color4;
    }
}
