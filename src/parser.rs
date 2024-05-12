use crate::tokenizer::InstructionType;
use crate::tokenizer::Token;

pub struct Parser {
    position: usize,
    read_position: usize,
    token: Token,
    input: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserToken {
    EOF,
    Invalid,
    BotInfo(Vec<String>),
    Instruction(Instruction),
    Data(Vec<Value>),
    Label(String),
    Comment,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand1: Operand,
    pub operand2: Operand,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Info {
    bot_name: String,
    author_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlusMinus {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(u16),
    Label(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    SP = 15,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum Operand {
    None,
    Direct(Value),                                                // 00 | [1234]
    Register(Register),                                           // 01 | r0
    ImmediateValue(Value),                                        // 10 | 1234
    RegisterIndexedDirect(Box<Operand>, PlusMinus, Box<Operand>), // 11 | [r0+100] OR [label+r0]
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        let mut parser = Parser {
            position: 0,
            read_position: 0,
            token: input[0].clone(),
            input,
        };

        parser.read_token();

        parser
    }

    fn read_token(&mut self) {
        println!("{:#?}", self.token);
        if !(self.token == Token::EOF) {
            self.token = self.input[self.read_position].clone();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_token(&mut self) -> Token {
        self.input[self.read_position + 1].clone()
    }

    fn read_data(&mut self) -> ParserToken {
        let mut data: Vec<Value> = Vec::new();
        self.read_token();
        self.read_token();

        while self.token != Token::CloseCurly {
            match self.token_to_operand() {
                Operand::ImmediateValue(val) => {
                    data.push(val);
                }
                _ => break,
            }
            self.read_token();
        }

        return ParserToken::Data(data);
    }

    fn read_label(&mut self) -> ParserToken {
        if let Token::Ident(label) = self.token.clone() {
            self.read_token();

            if self.token != Token::Colon {
                return ParserToken::Invalid;
            }

            ParserToken::Label(label)
        } else {
            ParserToken::Invalid
        }
    }

    fn u16_to_reg(reg: u16) -> Register {
        return match reg {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            15 => Register::SP,
            _ => Register::R0,
        };
    }

    fn token_to_operand(&mut self) -> Operand {
        match self.token.clone() {
            Token::Ident(ident) => {
                return Operand::ImmediateValue(Value::Label(ident));
            }
            Token::Number(int) => {
                return Operand::ImmediateValue(Value::Number(int));
            }
            Token::Register(reg) => {
                return Operand::Register(Parser::u16_to_reg(reg));
            }
            Token::StackPointer => return Operand::Register(Register::SP),
            _ => return Operand::None,
        }
    }

    fn token_to_sign(&mut self) -> PlusMinus {
        match self.token.clone() {
            Token::Plus => return PlusMinus::Plus,
            Token::Minus => return PlusMinus::Minus,
            _ => todo!(),
        }
    }

    fn read_operand(&mut self, instruction: InstructionType) -> Operand {
        match self.token.clone() {
            Token::Ident(_) | Token::Number(_) | Token::Register(_) | Token::StackPointer => {
                return self.token_to_operand()
            }
            Token::OpenBracket => {
                self.read_token(); // first operand

                let one = self.token_to_operand();

                self.read_token(); // second value (could be bracket or plus/minus)

                if self.token == Token::CloseBracket {
                    // not indexed memory access
                    match one {
                        Operand::Register(_) => {
                            // what is this, fucking lisp?
                            return Operand::RegisterIndexedDirect(
                                Box::new(one),
                                PlusMinus::Plus,
                                Box::new(Operand::ImmediateValue(Value::Number(0))),
                            );
                        }
                        Operand::ImmediateValue(value) => return Operand::Direct(value),
                        _ => return one,
                    }
                } else {
                    match self.token {
                        Token::Plus | Token::Minus => {
                            let sign = self.token_to_sign();

                            self.read_token(); // second operand

                            let two = self.token_to_operand();

                            self.read_token(); // read closing bracket

                            return Operand::RegisterIndexedDirect(
                                Box::new(one),
                                sign,
                                Box::new(two),
                            );
                        }
                        _ => {
                            println!("{:#?}", self.token);
                            todo!();
                        }
                    }
                }
            }
            _ => return Operand::None,
        }
    }

    fn read_instruction_single(&mut self, instruction: InstructionType) -> ParserToken {
        self.read_token();

        match self.token.clone() {
            Token::OpenBracket => {
                let instruction = Instruction {
                    instruction_type: instruction.clone(),
                    operand1: self.read_operand(instruction.clone()),
                    operand2: Operand::None,
                };

                return ParserToken::Instruction(instruction);
            }
            _ => {
                let instruction = Instruction {
                    instruction_type: instruction.clone(),
                    operand1: self.read_operand(instruction.clone()),
                    operand2: Operand::None,
                };

                ParserToken::Instruction(instruction)
            }
        }
    }

    fn read_instruction_double(&mut self, instruction: InstructionType) -> ParserToken {
        self.read_token();

        let op1 = self.read_operand(instruction.clone());
        self.read_token();
        self.read_token();
        let op2 = self.read_operand(instruction.clone());

        let instr = Instruction {
            instruction_type: instruction,
            operand1: op1,
            operand2: op2,
        };

        return ParserToken::Instruction(instr);
    }

    pub fn next_token(&mut self) -> ParserToken {
        let ptoken = match self.token.clone() {
            Token::EOF => ParserToken::EOF,
            Token::Invalid => ParserToken::Invalid,
            Token::Instruction(instruction) => match instruction {
                InstructionType::NOP | InstructionType::RET | InstructionType::EAT => {
                    let instruction = Instruction {
                        instruction_type: instruction,
                        operand1: Operand::None,
                        operand2: Operand::None,
                    };
                    ParserToken::Instruction(instruction)
                }

                InstructionType::PUSH
                | InstructionType::POP
                | InstructionType::CALL
                | InstructionType::JMP
                | InstructionType::JL
                | InstructionType::JLE
                | InstructionType::JG
                | InstructionType::JGE
                | InstructionType::JE
                | InstructionType::JNE
                | InstructionType::JS
                | InstructionType::JNS
                | InstructionType::ENERGY
                | InstructionType::TRAVEL
                | InstructionType::RELEASE
                | InstructionType::SENSE => self.read_instruction_single(instruction),

                InstructionType::MOV
                | InstructionType::ADD
                | InstructionType::SUB
                | InstructionType::MULT
                | InstructionType::DIV
                | InstructionType::MOD
                | InstructionType::AND
                | InstructionType::OR
                | InstructionType::XOR
                | InstructionType::CMP
                | InstructionType::TEST
                | InstructionType::GETXY
                | InstructionType::SHL
                | InstructionType::SHR
                | InstructionType::RAND
                | InstructionType::CHARGE
                | InstructionType::POKE
                | InstructionType::PEEK
                | InstructionType::CKSUM => self.read_instruction_double(instruction),
            },
            Token::Ident(ident) => match ident.to_lowercase().as_str() {
                "data" => self.read_data(),
                _ => self.read_label(),
            },
            Token::BotInfo(info) => ParserToken::BotInfo(info),
            Token::Comment => ParserToken::Comment,
            _ => ParserToken::Invalid,
        };

        self.read_token();

        return ptoken;
    }
}
