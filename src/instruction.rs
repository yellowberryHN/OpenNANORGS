use crate::tokenizer::InstructionType;

pub struct Instruction {
    bytecode: [u16; 3],
    op_code: InstructionType,
    op1: Operand,
    op2: Operand,
    instruction_pointer: u16,
}

impl Instruction {
    pub fn from_bytecode(bytecode: [u16; 3], instruction_pointer: u16) -> Instruction {
        //             opCode = (CPUOpCode)(this.bytecode[0] & 0xFF);
        let op_code = InstructionType::From((bytecode[0] & 0xFF) as u16);

        Instruction {
            bytecode,
            op_code: todo!(),
            op1: todo!(),
            op2: todo!(),
            instruction_pointer,
        }
    }
}

pub struct Operand {
    op_type: OperandType, // rust doesn't like using "type" as an identifier
    value: u16,
    offset: u16,
    sub: bool,
}

impl Operand {
    pub fn new(op_type: OperandType, value: u16, offset: u16, sub: bool) -> Operand {
        Operand {
            op_type,
            value,
            offset,
            sub,
        }
    }
}

pub enum OperandType {
    Direct = 0,          // 0 0
    Register = 1,        // 0 1
    Immediate = 2,       // 1 0
    RegisterIndexed = 3, // 1 1
}
