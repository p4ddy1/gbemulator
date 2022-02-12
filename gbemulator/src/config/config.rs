use crate::config::color_palette::ColorPalette;
use crate::config::controls::Controls;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub controls: Controls,
    #[serde(default = "ColorPalette::default")]
    pub color_palette: ColorPalette,
}

impl Config {
    pub fn create_from_toml(toml: &str) -> Result<Self, toml::de::Error> {
        let config: Config = toml::from_str(toml)?;
        Ok(config)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        let toml = toml::to_string_pretty(&self)?;
        Ok(toml)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            controls: Controls::default(),
            color_palette: ColorPalette::default(),
        }
    }
}
