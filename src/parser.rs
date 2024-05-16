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

impl From<Token> for PlusMinus {
    fn from(token: Token) -> PlusMinus {
        match token {
            Token::Plus => PlusMinus::Plus,
            Token::Minus => PlusMinus::Minus,
            _ => panic!("Tried to convert non-sign token into PlusMinus")
        }
    }
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


impl From<Register> for u16 {
    fn from(value: Register) -> Self {
        return match value {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::R8 => 8,
            Register::R9 => 9,
            Register::R10 => 10,
            Register::R11 => 11,
            Register::R12 => 12,
            Register::R13 => 13,
            Register::SP => 15,
        };
    }
}


impl From<u16> for Register {
    fn from(reg: u16) -> Self {
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

impl From<Token> for Operand {
    fn from(token: Token) -> Operand {
        match token {
            Token::Ident(ident) => {
                Operand::ImmediateValue(Value::Label(ident))
            }
            Token::Number(int) => {
                Operand::ImmediateValue(Value::Number(int))
            }
            Token::Register(reg) => {
                Operand::Register(Register::from(reg))
            }
            Token::StackPointer => return Operand::Register(Register::SP),
            _ => panic!("Token is not an operand"),
        }
    }
}

impl From<Operand> for u16 {
    fn from(operand: Operand) -> u16 {
        match operand {
            Operand::None => 0,
            Operand::Direct(_) => 0,
            Operand::Register(_) => 1,
            Operand::ImmediateValue(_) => 2,
            Operand::RegisterIndexedDirect(_, _, _) => 3,
        }
    }
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
            match Operand::from(self.token.clone()) {
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

    fn read_operand(&mut self, instruction: InstructionType) -> Operand {
        match self.token.clone() {
            Token::Ident(_) | Token::Number(_) | Token::Register(_) | Token::StackPointer => {
                return Operand::from(self.token.clone());
            }
            Token::OpenBracket => {
                self.read_token(); // first operand

                let one = Operand::from(self.token.clone());

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
                            let sign = PlusMinus::from(self.token.clone());

                            self.read_token(); // second operand

                            let two = Operand::from(self.token.clone());

                            self.read_token(); // read closing bracket

                            return Operand::RegisterIndexedDirect(
                                Box::new(one),
                                sign,
                                Box::new(two),
                            );
                        }
                        _ => {
                            panic!("Sign expected for register indexed direct addressing mode");
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
            Token::Instruction(instruction) => match instruction.get_operand_amount() {
                1 => self.read_instruction_single(instruction),
                2 => self.read_instruction_double(instruction),
                0 | _ => {
                    let instruction = Instruction {
                        instruction_type: instruction,
                        operand1: Operand::None,
                        operand2: Operand::None,
                    };
                    ParserToken::Instruction(instruction)
                }
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
