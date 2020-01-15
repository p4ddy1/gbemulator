use std::fs;

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        Cartridge { data }
    }

    pub fn new_from_file(filename: &str) -> Result<Cartridge, &str> {
        let data = match fs::read(filename) {
            Ok(data) => data,
            Err(_) => {
                return Err("Could not open file");
            }
        };

        Ok(Cartridge { data })
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}
