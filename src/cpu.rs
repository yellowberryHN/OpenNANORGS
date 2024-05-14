use crate::disassembler::Disassembler;
use crate::emulator::Bot;
use crate::tokenizer::InstructionType;

#[derive(Debug, Clone)]
pub struct CPU {
    instruction_pointer: u16,
    stack_pointer: u16,
    registers: [u16; 14],
    program_memory: [u16; 3600],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            instruction_pointer: 0,
            stack_pointer: 3600,
            registers: [0u16; 14],
            program_memory: [0u16; 3600],
        }
    }

    pub fn flash(&mut self, bytecode: Vec<u16>) {
        self.program_memory = [0u16; 3600];

        for (pos, word) in bytecode.iter().enumerate() {
            self.program_memory[pos] = *word;
        }
    }

    pub fn tick(&mut self, bot: &mut Bot) {
        // TODO: this is where instructions will be ran
        let instruction = self.get_instruction();
        let instruction_type = InstructionType::from(instruction[0] & 0xFF);

        println!("{}", Disassembler::parse(instruction, self.instruction_pointer, false));

        match instruction_type {
            InstructionType::NOP => {self.increment_ip();},
            _ => {self.increment_ip();}
        }

        bot.energy -= 1;
        bot.cpu = self.clone();
    }

    fn set_instruction_pointer(&mut self, ip: u16) {
        self.instruction_pointer = ip % self.program_memory.len() as u16;
        self.instruction_pointer -= self.instruction_pointer % 3;
    }

    fn increment_ip(&mut self) {
        self.set_instruction_pointer(self.instruction_pointer + 3)
    }

    fn get_instruction(&self) -> [u16; 3] {
        let end = (self.instruction_pointer as usize) + 3;
        let slice = &self.program_memory[(self.instruction_pointer as usize)..end];
        <[u16; 3]>::try_from(slice).expect("Instruction should have exactly 3 words")
    }
}
