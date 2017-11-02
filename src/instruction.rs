pub enum Instruction {
    Immediate { name: Name, op_code: u8 },
    ZeroPage { name: Name, op_code: u8 },
    ZeroPageX { name: Name, op_code: u8 },
    Absolute { name: Name, op_code: u8 },
    AbsoluteX { name: Name, op_code: u8 },
    AbsoluteY { name: Name, op_code: u8 },
    IndirectX { name: Name, op_code: u8 },
    IndirectY { name: Name, op_code: u8 },
}

pub enum Name {
    CLC,
    CLD,
    CLI,
    CLV,
    DEX,
    DEY,
    INX,
    INY,
    NOP,
    SEC,
    SED,
    SEI,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    UNKNOWN,
}

pub fn to_instruction(string: &'static str) -> Instruction {
    match string {
        "clc" => Instruction::Immediate {
            name: Name::CLC,
            op_code: 0x18,
        },
        "cld" => Instruction::Immediate {
            name: Name::CLD,
            op_code: 0xD8,
        },
        "cli" => Instruction::Immediate {
            name: Name::CLI,
            op_code: 0x58,
        },
        "clv" => Instruction::Immediate {
            name: Name::CLV,
            op_code: 0xB8,
        },
        "dex" => Instruction::Immediate {
            name: Name::DEX,
            op_code: 0xCA,
        },
        "dey" => Instruction::Immediate {
            name: Name::DEY,
            op_code: 0x88,
        },
        "inx" => Instruction::Immediate {
            name: Name::INX,
            op_code: 0xE8,
        },
        "iny" => Instruction::Immediate {
            name: Name::INY,
            op_code: 0xC8,
        },
        "nop" => Instruction::Immediate {
            name: Name::NOP,
            op_code: 0xEA,
        },
        "sec" => Instruction::Immediate {
            name: Name::SEC,
            op_code: 0x38,
        },
        "sed" => Instruction::Immediate {
            name: Name::SED,
            op_code: 0xF8,
        },
        "sei" => Instruction::Immediate {
            name: Name::SEI,
            op_code: 0x78,
        },
        "tax" => Instruction::Immediate {
            name: Name::TAX,
            op_code: 0xAA,
        },
        "tay" => Instruction::Immediate {
            name: Name::TAY,
            op_code: 0xA8,
        },
        "tsx" => Instruction::Immediate {
            name: Name::TSX,
            op_code: 0xBA,
        },
        "txa" => Instruction::Immediate {
            name: Name::TXA,
            op_code: 0x8A,
        },
        "txs" => Instruction::Immediate {
            name: Name::TXS,
            op_code: 0x9A,
        },
        "tya" => Instruction::Immediate {
            name: Name::TYA,
            op_code: 0x98,
        },
        _ => Instruction::Immediate {
            name: Name::UNKNOWN,
            op_code: 0x00,
        },
    }
}
