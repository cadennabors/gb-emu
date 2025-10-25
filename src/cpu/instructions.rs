#[derive(Copy, Clone)]
pub enum Instruction {
    UNDEFINED,

}

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    Immediate16,
    Register,
    Register16,
    RegisterIndirect,
    DirectAddress,
    DirectAddress8,
    SP,
    Implied,
}

macro_rules! op {
    ($instr:ident, $mode:ident, $cycl:literal) => {
        Operation { instruction: Instruction::$instr, addressing_mode: AddressingMode::$mode, cycles: $cycl }
    };
}

#[derive(Copy, Clone)]
struct Operation {
instruction : Instruction,
addressing_mode : AddressingMode,
cycles : u8,
}

pub const OPCODE_TABLE : [Operation;256] = [
    op!(UNDEFINED, Immediate, 3); 256
];