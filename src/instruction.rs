pub enum Instruction {
    DEX = 0xCA,
    DEY = 0x88,
    INX = 0xE8,
    INY = 0xC8,
    TAX = 0xAA,
    TAY = 0xA8,
    UNKNOWN = 0x00,
}

pub fn to_instruction(string: &'static str) -> Instruction {
    match string {
        "dex" => Instruction::DEX,
        "dey" => Instruction::DEY,
        "inx" => Instruction::INX,
        "iny" => Instruction::INY,
        "tax" => Instruction::TAX,
        "tay" => Instruction::TAY,
        _ => Instruction::UNKNOWN,
    }
}
