pub fn bytes_to_word(byte1: u8, byte2: u8) -> u16 {
    ((byte1 as u16) << 8) + byte2 as u16
}

pub fn word_to_bytes(word: u16) -> (u8, u8) {
    ((word >> 8) as u8, word as u8)
}

pub fn is_bit_set(byte: &u8, index: u8) -> bool {
    (1 << index) & byte > 0
}

pub fn reset_bit_in_byte(byte: u8, bit_index: u8) -> u8 {
    byte & ((1 << bit_index) ^ 0xFF)
}

pub fn set_bit_in_byte(byte: u8, bit_index: u8) -> u8 {
    byte | 1 << bit_index
}
