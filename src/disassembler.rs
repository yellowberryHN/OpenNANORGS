use crate::tokenizer::InstructionType;

pub struct Disassembler {
    bytecode: Vec<u16>,
}

impl Disassembler {
    pub fn new(bytecode: Vec<u16>) -> Disassembler {
        let disassembler = Disassembler {
            bytecode
        };

        disassembler
    }

    pub fn print_disassembly(&self, bot_name: String) {
        print!("Disassembly of {bot_name}:\n\n");
        let instruction_count: u16 = (self.bytecode.len() / 3) as u16;

        for i in 0..instruction_count {
            let ip = i*3;
            let instruction = self.get_instruction(ip);

            // who let me cook
            let byte_string = instruction.iter()
                .map(|&num| format!("{:04X}", num))
                .collect::<Vec<_>>().join(" ");

            println!("{:04}  {:<30} ({})", ip, Self::parse(instruction, ip, true), byte_string);
        }
    }

    fn get_instruction(&self, position: u16) -> [u16;3] {
        let end = (position as usize) + 3;
        let slice = &self.bytecode[(position as usize)..end];
        <[u16; 3]>::try_from(slice).expect("Instruction should have exactly 3 words")
    }

    pub fn parse_at(&self, ip: u16, data_allowed: bool) -> String {
        Self::parse(self.get_instruction(ip), ip, data_allowed)
    }

    pub fn parse(bytes: [u16; 3], instruction_pointer: u16, data_allowed: bool) -> String {
        let mut result = String::new();

        let is_data = bytes[0] & 0xFF > InstructionType::CKSUM as u16;

        if is_data && data_allowed {
            return format!("data {{ {} {} {} }}", bytes[0], bytes[1], bytes[2]);
        } else {
            let instruction_type = InstructionType::from(bytes[0] & 0xFF);

            result += match instruction_type {
                InstructionType::NOP => "nop",
                InstructionType::MOV => "mov",
                InstructionType::PUSH => "push",
                InstructionType::POP => "pop",
                InstructionType::CALL => "call",
                InstructionType::RET => "ret",
                InstructionType::JMP => "jmp",
                InstructionType::JL => "jl",
                InstructionType::JLE => "jle",
                InstructionType::JG => "jg",
                InstructionType::JGE => "jge",
                InstructionType::JE => "je",
                InstructionType::JNE => "jne",
                InstructionType::JS => "js",
                InstructionType::JNS => "jns",
                InstructionType::ADD => "add",
                InstructionType::SUB => "sub",
                InstructionType::MULT => "mult",
                InstructionType::DIV => "div",
                InstructionType::MOD => "mod",
                InstructionType::AND => "and",
                InstructionType::OR => "or",
                InstructionType::XOR => "xor",
                InstructionType::CMP => "cmp",
                InstructionType::TEST => "test",
                InstructionType::GETXY => "getxy",
                InstructionType::ENERGY => "energy",
                InstructionType::TRAVEL => "travel",
                InstructionType::SHL => "shl",
                InstructionType::SHR => "shr",
                InstructionType::SENSE => "sense",
                InstructionType::EAT => "eat",
                InstructionType::RAND => "rand",
                InstructionType::RELEASE => "release",
                InstructionType::CHARGE => "charge",
                InstructionType::POKE => "poke",
                InstructionType::PEEK => "peek",
                InstructionType::CKSUM => "cksum",
            };

            let ops = instruction_type.get_operand_amount();

            if ops >= 1 {
                result += " ";

                let op1 = bytes[1];
                let op1_type = bytes[0] >> 12 >> 2 & 0x3;
                let op1_neg = bytes[0] >> 11 & 0x1 == 1;
                let op1_pos = instruction_type.is_positional();

                result += Self::parse_operand(op1, op1_type, instruction_pointer, op1_pos, op1_neg).as_str();
            }
            if ops == 2 {
                result += ", ";

                let op2 = bytes[2];
                let op2_type = bytes[0] >> 12 & 0x3;
                let op2_neg = bytes[0] >> 10 & 0x1 == 1;

                result += Self::parse_operand(op2, op2_type, instruction_pointer, false, op2_neg).as_str();
            }
        }

        result
    }

    fn parse_operand(value: u16, _type: u16, instruction_pointer: u16, positional: bool, negative_offset: bool) -> String {
        match _type {
            0 => format!("[{value}]"),
            1 => {
                match value {
                    15 => "sp".to_string(),
                    _ => format!("r{value}")
                }
            },
            2 => {
                if positional {
                    instruction_pointer.wrapping_add(value).to_string()
                } else {
                    value.to_string()
                }
            },
            3 => {
                let offset = value & 0xFFF;

                let register = match value >> 12 {
                    15 => "sp".to_string(),
                    _ => format!("r{}", value >> 12)
                };

                if offset > 0 {
                    if negative_offset {
                        format!("[{}-{}]", register, 0x1000u16 - offset)
                    } else {
                        format!("[{}+{offset}]", register)
                    }
                } else {
                    format!("[{}]", register)
                }
            },
            _ => value.to_string()
        }
    }
}