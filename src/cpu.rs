struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_16_bit_register(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    fn set_16_bit_register(first_register : &mut u8, second_register : &mut u8, val : u16) {
        *first_register = ((val & 0xFF00) >> 8) as u8;
        *second_register = (val & 0x00FF) as u8;
    }
}