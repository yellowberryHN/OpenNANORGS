use std::collections::HashMap;
use crate::tokenizer::{Tokenizer, InstructionType};
use crate::parser::{Instruction, Operand, ParserToken, PlusMinus, Register, Value};

pub struct Compiler {
    position: usize,
    read_position: usize,
    byte_position: usize,
    instruction: ParserToken,
    input: Vec<ParserToken>,
    labels: Vec<(String, u16)>,
    label_index: Vec<(String, (u16, u16))>,
    symbol_table: HashMap<String, u16>,
    pub output: Vec<u16>,
}

impl Compiler {
    pub fn new(input: Vec<ParserToken>, symbol_table: HashMap<String, u16>) -> Compiler {
        let mut compiler = Compiler {
            position: 0,
            read_position: 0,
            byte_position: 0,
            instruction: ParserToken::Invalid,
            output: Vec::new(),
            label_index: Vec::new(),
            input: input.clone(),
            labels: Vec::new(),
            symbol_table
        };

        compiler.read_instruction();
        compiler
    }

    pub fn compile(&mut self) {
        let mut bytecode: Vec<u16> = vec![];
        let mut instruction_pointer = 0;

        for token in &self.input {
            match token {
                ParserToken::Instruction(instruction) => {
                    while (instruction_pointer % 3) != 0 {
                        bytecode.push(0);
                        instruction_pointer += 1;
                    }

                    let op1 = &instruction.operand1;
                    let op2 = &instruction.operand2;

                    let mut op1_value: u16 = 0;
                    let mut op2_value: u16 = 0;

                    let mut op1_offset: u16 = 0;
                    let mut op2_offset: u16 = 0;

                    let positional = match instruction.instruction_type {
                        InstructionType::CALL => true,
                        InstructionType::JMP
                        | InstructionType::JL
                        | InstructionType::JLE
                        | InstructionType::JG
                        | InstructionType::JGE
                        | InstructionType::JE
                        | InstructionType::JNE
                        | InstructionType::JS
                        | InstructionType::JNS => true,
                        _ => false,
                    };

                    match op1 {
                        Operand::None => {}
                        Operand::Direct(value)
                        | Operand::ImmediateValue(value) => {
                            match value {
                                Value::Number(num) => op1_value = *num,
                                Value::Label(label) => {
                                    op1_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();
                                    if positional {
                                        op1_value = op1_value.wrapping_sub(instruction_pointer);
                                    }
                                }
                            }
                        }
                        Operand::Register(register) => {
                            op1_value = register.to_owned() as u16;
                        }
                        // TODO: WIP, not even implemented in the tokenizer yet
                        Operand::RegisterIndexedDirect(base, operator, offset) => {
                            match base.as_ref() {
                                Operand::ImmediateValue(value) => {
                                    match value {
                                        Value::Label(label) => {
                                            op1_value = *self.symbol_table.get(label).unwrap();
                                        }
                                        _ => {}
                                    }
                                }
                                Operand::Register(register) => {
                                    op1_value = register.to_owned() as u16;
                                }
                                _ => {}
                            }
                            match operator {
                                PlusMinus::Plus => {}
                                PlusMinus::Minus => {}
                            }
                            match offset.as_ref() {
                                Operand::Direct(value) => {}
                                Operand::Register(register) => {}
                                _ => {}
                            }
                        }
                    }

                    match op2 {
                        Operand::None => {}
                        Operand::Direct(value)
                        | Operand::ImmediateValue(value) => {
                            match value {
                                Value::Number(num) => op2_value = *num,
                                Value::Label(label) => {
                                    op2_value = *self.symbol_table.get(&label.to_lowercase()).unwrap();
                                    if positional {
                                        op2_value = op2_value.wrapping_sub(instruction_pointer);
                                    }
                                }
                            }
                        }
                        Operand::Register(register) => {
                            op2_value = register.to_owned() as u16;
                        }
                        // TODO: WIP above
                        Operand::RegisterIndexedDirect(_,_,_) => {}
                    }

                    let mut inst = instruction.to_owned().instruction_type as u16 | (Compiler::get_modes(instruction) << 12);

                    bytecode.push(inst);
                    bytecode.push(op1_value | (op1_offset << 12));
                    bytecode.push(op2_value | (op2_offset << 12));
                    instruction_pointer += 3;
                }
                ParserToken::Data(data) => {
                    for value in data {
                        bytecode.push(match value {
                            Value::Number(num) => *num,
                            Value::Label(label) => *self.symbol_table.get(&label.to_lowercase()).unwrap()
                        });
                        instruction_pointer += 1;
                    }
                }
                _ => {}
            }
        }

        self.output = bytecode;
    }

    fn get_modes(instruction: &Instruction) -> u16 {
        /*
        ushort value = 0;
        value = (ushort)((ushort)op1mode << 2); // what the fuck
        value |= (ushort)op2mode;

        return value;
        */
        let mut value = 0;
        value = Compiler::operand_to_mode_val(instruction.to_owned().operand1) << 2;
        value |= Compiler::operand_to_mode_val(instruction.to_owned().operand2);

        value
    }

    fn operand_to_mode_val(operand: Operand) -> u16 {
        match operand {
            Operand::None => 0,
            Operand::Direct(_) => 0,
            Operand::Register(_) => 1,
            Operand::ImmediateValue(_) => 2,
            Operand::RegisterIndexedDirect(_, _, _) => 3,
        }
    }

    fn read_instruction(&mut self) {
        if !(self.instruction == ParserToken::EOF) {
            self.instruction = self.input[self.read_position].clone();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}