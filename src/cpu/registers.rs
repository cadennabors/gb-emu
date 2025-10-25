#![allow(dead_code)]
pub struct Registers {
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

pub enum Reg8b {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Reg16b {
    AF,
    BC,
    DE,
    HL,
}

enum Flags {
    Zero,
    Carry,
    Subtraction,
    HalfCarry,
}

impl Registers {
    fn new() -> Self {
        Registers {
            pc: 0x00,
            sp: 0x00,
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            f: 0x0,
            h: 0x0,
            l: 0x0,
        }
    }

    fn combine_bytes(hi: u8, lo: u8) -> u16 {
        ((hi as u16) << 8) | (lo as u16)
    }

    fn split_bytes(val: u16) -> (u8, u8) {
        let hi = ((val & 0xFF00) >> 8) as u8;
        let lo = (val & 0x00FF) as u8;
        (hi, lo)
    }

    fn get_16(&self, register: Reg16b) -> u16 {
        let (hi, lo) = match register {
            Reg16b::AF => (self.a, self.f),
            Reg16b::BC => (self.b, self.c),
            Reg16b::DE => (self.d, self.e),
            Reg16b::HL => (self.h, self.l),
        };
        Self::combine_bytes(hi, lo)
    }

    fn set_16(&mut self, register: Reg16b, val: u16) {
        let (hi, lo) = Self::split_bytes(val);
        match register {
            Reg16b::AF => {
                self.a = hi;
                self.f = lo;
            }
            Reg16b::BC => {
                self.b = hi;
                self.c = lo;
            }
            Reg16b::DE => {
                self.d = hi;
                self.e = lo;
            }
            Reg16b::HL => {
                self.h = hi;
                self.l = lo;
            }
        }
    }
}
