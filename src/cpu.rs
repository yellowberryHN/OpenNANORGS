#[derive(Debug)]
pub struct CPU {
    instruction_pointer: u16,
    stack_pointer: u16,
    registers: [u16; 14],
    program_memory: [u16; 3600],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            instruction_pointer: todo!(),
            stack_pointer: todo!(),
            registers: todo!(),
            program_memory: todo!(),
        }
    }

    fn get_instruction() {}
}
