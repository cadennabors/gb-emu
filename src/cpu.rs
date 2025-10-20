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
    //todo : add 16 bit get/set functions rather than one for each combination
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    fn set_af(&mut self, val : u16) {
        self.a = ((val & 0xFF00) >> 8) as u8;
        self.f = (val & 0x00FF) as u8;
    }
}