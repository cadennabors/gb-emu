struct Registers {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

enum Reg16b {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    fn combine_bytes(hi: &u8, lo: &u8) -> u16 {
        ((*hi as u16) << 8) | (*lo as u16)
    }
    fn split_bytes(hi: &mut u8, lo: &mut u8, val: u16) {
        *hi = ((val & 0xFF00) >> 8) as u8;
        *lo = (val & 0x00FF) as u8;
    }

    fn get_16(&self, register: Reg16b) -> u16 {
        match register {
            Reg16b::AF => Self::combine_bytes(&self.a, &self.f),
            Reg16b::BC => Self::combine_bytes(&self.b, &self.c),
            Reg16b::DE => Self::combine_bytes(&self.d, &self.e),
            Reg16b::HL => Self::combine_bytes(&self.h, &self.l),
        }
    }
}
