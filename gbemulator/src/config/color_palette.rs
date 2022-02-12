use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorPalette {
    pub color1: [u8; 3],
    pub color2: [u8; 3],
    pub color3: [u8; 3],
    pub color4: [u8; 3],
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette {
            color1: [8, 24, 32],
            color2: [52, 104, 86],
            color3: [136, 192, 112],
            color4: [224, 248, 208],
        }
    }
}
