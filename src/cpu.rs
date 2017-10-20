use ram::Ram;
use instruction::Instruction;

#[derive(PartialEq, Debug)]
pub struct Cpu {
    pub cycles: u64,
    program_counter: u16,
    stack_pointer: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    ram: Ram,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            cycles: 0,
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            status: 0,
            ram: Ram::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.cycles += 2;
                self.status = calculate_status(self.x)
            }
            Instruction::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.cycles += 2;
                self.status = calculate_status(self.y)
            }
            Instruction::INX => {
                self.x = self.x.wrapping_add(1);
                self.cycles += 2;
                self.status = calculate_status(self.x)
            }
            Instruction::INY => {
                self.y = self.y.wrapping_add(1);
                self.cycles += 2;
                self.status = calculate_status(self.y)
            }
            Instruction::TAX => {
                self.x = self.accumulator;
                self.cycles += 2;
                self.status = calculate_status(self.x)
            }
            Instruction::TAY => {
                self.y = self.accumulator;
                self.cycles += 2;
                self.status = calculate_status(self.y)
            }
            _ => {}
        }
    }
}

fn calculate_status(new: u8) -> u8 {
    zero(new) | negative(new)
}

fn negative(new: u8) -> u8 {
    new & 0b100000
}

fn zero(new: u8) -> u8 {
    return if new == 0 { 0b00010 } else { 0 };
}