use lib_gbemulation::cartridge::RamDumper;
use std::fs;

pub struct FilesystemRamDumper {
    filename: String,
}

impl FilesystemRamDumper {
    pub fn new(rom_filename: &String) -> Self {
        let rom_name = if rom_filename.ends_with(".gb") {
            &rom_filename[..rom_filename.len() - 3]
        } else {
            rom_filename
        };

        let savegame_filename = format!("{}.sav", rom_name);

        FilesystemRamDumper {
            filename: savegame_filename,
        }
    }
}

impl RamDumper for FilesystemRamDumper {
    fn dump(&self, data: &Vec<u8>) {
        fs::write(&self.filename, data).unwrap();
    }

    fn load(&self) -> Option<Vec<u8>> {
        if let Ok(data) = fs::read(&self.filename) {
            return Some(data);
        }

        None
    }
}
