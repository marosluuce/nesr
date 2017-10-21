pub enum Instruction {
    CLC = 0x18,
    DEX = 0xCA,
    DEY = 0x88,
    INX = 0xE8,
    INY = 0xC8,
    NOP = 0xEA,
    TAX = 0xAA,
    TAY = 0xA8,
    TSX = 0xBA,
    TXA = 0x8A,
    TXS = 0x9A,
    TYA = 0x98,
    UNKNOWN = 0x00,
}

pub fn to_instruction(string: &'static str) -> Instruction {
    match string {
        "clc" => Instruction::CLC,
        "dex" => Instruction::DEX,
        "dey" => Instruction::DEY,
        "inx" => Instruction::INX,
        "iny" => Instruction::INY,
        "nop" => Instruction::NOP,
        "tax" => Instruction::TAX,
        "tay" => Instruction::TAY,
        "tsx" => Instruction::TSX,
        "txa" => Instruction::TXA,
        "txs" => Instruction::TXS,
        "tya" => Instruction::TYA,
        _ => Instruction::UNKNOWN,
    }
}
