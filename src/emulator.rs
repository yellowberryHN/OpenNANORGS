use std::cmp::PartialEq;
use crate::parser::{Operand, Register, Value};
use crate::rng::{LegacyRNG, ModernRNG, RNGSystem};
use crate::tokenizer::InstructionType;
use ruscii::spatial::Vec2;
use std::fmt::Formatter;
use std::thread::current;

#[derive(Debug)]
pub enum ItemType {
    Sludge,
    CollectionPoint,
    Ramp,
}

#[derive(Debug)]
pub struct Item {
    pub id: u16,
    pub position: Position,
    pub item_type: ItemType,
}

impl Item {
    pub fn get_glyph(&self) -> char {
        match self.item_type {
            ItemType::Sludge => '*',
            ItemType::CollectionPoint => '$',
            ItemType::Ramp => '/',
        }
    }
}

#[derive(Debug)]
pub struct Tank {
    bounds: Position,
    pub score: u64,
    sludge_types: u8,
    toxic_sludge: Vec<u8>,
    pub elements: Vec<Option<Item>>,
}

impl Tank {
    pub fn new(bounds: Position) -> Tank {
        let mut tank = Tank {
            score: 0,
            sludge_types: 0,
            toxic_sludge: vec![],
            elements: vec![],
            bounds,
        };
        tank.elements.resize_with(
            usize::from(tank.bounds.x) * usize::from(tank.bounds.y) * usize::from(tank.bounds.z),
            Default::default,
        );
        tank
    }

    fn get_index(&self, pos: &Position) -> usize {
        //println!("checking index of {:?}", pos);
        usize::from(pos.x)
            + usize::from(pos.y) * usize::from(self.bounds.x)
            + usize::from(pos.z) * usize::from(self.bounds.x) * usize::from(self.bounds.y)
    }

    pub fn is_occupied(&self, pos: &Position) -> bool {
        let index = self.get_index(pos);
        self.elements[index].is_some()
    }

    fn add_item(&mut self, item_type: ItemType, id: u16, pos: Position) {
        let index = self.get_index(&pos);
        self.elements[index] = Some(Item {
            id,
            position: pos,
            item_type,
        });
    }

    fn eat_item(&mut self, pos: &Position) -> bool {
        let index = self.get_index(pos);
        match &self.elements[index] {
            Some(item) => {
                match item.item_type {
                    ItemType::Sludge => {
                        self.elements[index] = None;
                        return false; // TODO: return toxicity
                    }
                    _ => panic!("this shouldn't happen")
                }
            }
            None => false
        }
    }

    pub fn get_random_position(&self, rng: &mut Box<dyn RNGSystem>) -> Position {
        loop {
            let pos = Position {
                x: rng.rand(Some((self.bounds.x - 1) as u32)) as u8,
                y: rng.rand(Some((self.bounds.y - 1) as u32)) as u8,
                z: rng.rand(Some((self.bounds.z - 1) as u32)) as u8,
            };

            if !self.is_occupied(&pos) {
                return pos;
            }
        }
    }

    pub fn initial_fill(&mut self, num_items: usize, rng: &mut Box<dyn RNGSystem>) {
        self.sludge_types = rng.rand(32.into()) as u8;

        for _ in 0..num_items {
            let pos = self.get_random_position(rng);
            let rand_id = rng.rand(Some(self.sludge_types as u32)) as u16;
            self.add_item(ItemType::Sludge, rand_id, pos);
        }
    }

    fn get_item(&self, pos: &Position) -> Option<&Item> {
        let index = self.get_index(pos);
        self.elements[index].as_ref()
    }

    pub fn deposit(&mut self, amount: u16, pos: &Position) -> bool {
        match self.get_item(pos) {
            Some(item) => {
                match item.item_type {
                    ItemType::Sludge => {
                        self.score += amount as u64;
                        return true;
                    }
                    _ => false
                }
            }
            None => false
        }
    }
}

#[derive(Debug)]
pub struct Bot {
    pub id: u16,
    pub position: Position,
    pub energy: u16,
    pub sleeping: bool,

    // CPU related
    pub instruction_pointer: u16,
    stack_pointer: u16,
    pub registers: [u16; 14],
    program_memory: [u16; 3600],
    pub flags: CPUFlags,
}
#[derive(Debug)]
pub struct CPUFlags {
    pub success: bool,
    pub less: bool,
    pub equal: bool,
    pub greater: bool,
}

impl CPUFlags {
    fn new() -> CPUFlags {
        CPUFlags {
            success: false,
            less: false,
            equal: false,
            greater: false,
        }
    }

    fn clear(&mut self) {
        self.success = false;
        self.less = false;
        self.equal = false;
        self.greater = false;
    }
}

impl std::fmt::Display for CPUFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut result = String::new();

        if self.equal {
            result += "e"
        } else if self.less {
            result += "l"
        } else if self.greater {
            result += "g"
        }

        if self.success {
            result += "s"
        }

        write!(f, "{}", result)
    }
}

macro_rules! simple_math_instr {
    ($idx:expr, $src:expr, $dest:expr, $bots: expr, $op:tt) => {
        {
            let idx: usize = $idx;
            let src: Operand = $src;
            let dest: Operand = $dest;
            let bots: &mut Vec<Bot> = $bots;

            let value = bots[idx].get(&dest) $op bots[idx].get(&src);
            bots[idx].put(&dest, value);
            bots[idx].energy -= 1;
            bots[idx].increment_ip();
        }
    };
}

// Bot Helpers
impl Bot {
    pub fn new(id: u16, position: Position) -> Bot {
        Bot {
            id,
            position,
            energy: 10000,
            sleeping: false,

            instruction_pointer: 0,
            stack_pointer: 3600,
            registers: [0u16; 14],
            program_memory: [0u16; 3600],
            flags: CPUFlags::new(),
        }
    }

    pub fn flash(&mut self, bytecode: Vec<u16>) {
        self.program_memory = [0u16; 3600];

        for (pos, word) in bytecode.iter().enumerate() {
            self.program_memory[pos] = *word;
        }
    }

    pub fn flash_drone(&mut self) {
        let malicious: Vec<u16> = vec![
            0x8004, 0x000F, 0x0000, 0x8004, 0x0018, 0x0000, 0x8004, 0x002A, 0x0000, 0x8004, 0x0045,
            0x0000, 0x8006, 0xFFF7, 0x0000, 0x2020, 0x0DFB, 0x0004, 0x2020, 0x0DFC, 0x000A, 0x200F,
            0x0DFC, 0x0001, 0x0005, 0x0000, 0x0000, 0x401E, 0x0002, 0x0000, 0x800E, 0x000F, 0x0000,
            0x401A, 0x0002, 0x0000, 0x6017, 0x0002, 0x2710, 0x8009, 0x0006, 0x0000, 0x001F, 0x0000,
            0x0000, 0x0005, 0x0000, 0x0000, 0x4001, 0x0000, 0x0DFB, 0x6020, 0x0001, 0x0DF8, 0x5024,
            0x0000, 0x0001, 0x800E, 0x0012, 0x0000, 0x7017, 0x0000, 0x1000, 0x800B, 0x000C, 0x0000,
            0x6020, 0x0001, 0x0E10, 0x7001, 0x0000, 0x1000, 0x1023, 0x0DFB, 0x0001, 0x0005, 0x0000,
            0x0000, 0x2017, 0x0DFC, 0x0000, 0x800B, 0x000F, 0x0000, 0x001B, 0x0DFB, 0x0000, 0x800E,
            0x0009, 0x0000, 0x2010, 0x0DFC, 0x0001, 0x0005, 0x0000, 0x0000, 0x8004, 0xFFAF, 0x0000,
            0x8006, 0xFFEB, 0x0000,
        ];

        self.flash(malicious);
    }

    pub fn get_glyph(&self) -> char {
        if !self.sleeping {
            match self.id {
                0..=26 => ((self.id + 64) as u8).into(),
                27..=50 => ((self.id + 70) as u8).into(),
                _ => '@',
            }
        } else {
            match self.id {
                0..=50 => '.',
                _ => ',',
            }
        }
    }

    pub fn id_from_glyph(glyph: char) -> u16 {
        match glyph as u8 {
            64..=90 => (glyph as u8 - 64).into(),
            97..=120 => (glyph as u8 - 70).into(),
            _ => 0u16,
        }
    }

    pub fn has_energy(&self, amount: u16) -> bool {
        self.energy >= amount
    }

    pub fn is_occupied(pos: &Position, bots: &Vec<Bot>) -> bool {
        Bot::occupied_by(pos, bots) != 0xFFFF
    }

    pub fn occupied_by(pos: &Position, bots: &Vec<Bot>) -> u16 {
        for bot in bots {
            if bot.position == *pos {
                return bot.id - 1;
            }
        }
        0xFFFFu16
    }
}

// Bot CPU
impl Bot {
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

    fn get(&self, src: &Operand) -> u16 {
        match src {
            Operand::None => panic!("Cannot get from invalid operand"),
            Operand::Direct(value) => match value {
                Value::Number(value) => {
                    if *value < 3600 {
                        self.program_memory[*value as usize]
                    } else {
                        0
                    }
                }
                _ => panic!("Direct address was not Number"),
            },
            Operand::Register(reg) => match reg {
                Register::SP => self.stack_pointer,
                _ => self.registers[u16::from(reg.clone()) as usize],
            },
            Operand::ImmediateValue(value) => match value {
                Value::Number(value) => *value,
                _ => panic!("Immediate value was not Number"),
            },
            Operand::RegisterIndexedDirect(_, _, _) => todo!(),
        }
    }

    fn put(&mut self, dest: &Operand, value: u16) {
        match dest {
            Operand::None => {}
            Operand::Direct(vl) => match vl {
                Value::Number(vl) => {
                    if *vl < 3600 {
                        self.program_memory[*vl as usize] = value
                    }
                }
                _ => {}
            },
            Operand::Register(reg) => match reg {
                Register::SP => self.stack_pointer = value,
                _ => self.registers[u16::from(reg.clone()) as usize] = value,
            },
            Operand::ImmediateValue(_) => panic!("Attempt to put something into immediate value"),
            Operand::RegisterIndexedDirect(_, _, _) => todo!(),
        };
    }

    fn jump_to(&mut self, to: Operand) {
        let location = self.get(&to);

        match to {
            Operand::ImmediateValue(vl) => match vl {
                Value::Number(_) => {
                    self.set_instruction_pointer(self.instruction_pointer.wrapping_add(location))
                }
                _ => panic!("Immediate value was not Number"),
            },
            _ => self.set_instruction_pointer(location),
        }
    }

    fn mutate(&mut self, rng: &mut Box<dyn RNGSystem>) {
        let index = rng.rand(Some(self.program_memory.len() as u32)) as usize;
        let value = rng.rand(Some(0x1_0000)) as u16;
        self.program_memory[index] = value;
    }

    // TODO: this needs bounds handling, probably
    fn push(&mut self, value: u16) {
        self.stack_pointer -= 1;
        self.program_memory[self.stack_pointer as usize] = value;
    }

    // TODO: this needs bounds handling, probably
    fn pop(&mut self) -> u16 {
        let result = self.program_memory[self.stack_pointer as usize];
        self.stack_pointer += 1;

        result
    }

    pub fn tick(idx: usize, tank: &mut Tank, bots: &mut Vec<Bot>, rng: &mut Box<dyn RNGSystem>) {
        // i'm tired
        if bots[idx].energy < 1 {
            bots[idx].sleeping = true;
            return;
        } else {
            bots[idx].sleeping = false;
        }

        // TODO: this is where instructions will be ran
        let instruction = bots[idx].get_instruction();
        let instruction_id = instruction[0] & 0xFF;

        //println!("{}", Disassembler::parse(instruction, bots[idx].instruction_pointer, false));
        if instruction_id <= InstructionType::CKSUM as u16 {
            let op1_value = instruction[1];
            let op2_value = instruction[2];

            let op1_type = instruction[0] >> 14 & 0x3;
            let op2_type = instruction[0] >> 12 & 0x3;

            let op1 = match op1_type {
                0 => Operand::Direct(Value::Number(op1_value)),
                1 => Operand::Register(op1_value.into()),
                2 => Operand::ImmediateValue(Value::Number(op1_value)),
                3 => {
                    todo!("I don't feel like doing this right now.");
                    //Operand::RegisterIndexedDirect(_, _, _)
                }
                _ => panic!("Unknown addressing mode"),
            };

            let op2 = match op2_type {
                0 => Operand::Direct(Value::Number(op2_value)),
                1 => Operand::Register(op2_value.into()),
                2 => Operand::ImmediateValue(Value::Number(op2_value)),
                3 => {
                    todo!("I don't feel like doing this right now.");
                    //Operand::RegisterIndexedDirect(_, _, _)
                }
                _ => panic!("Unknown addressing mode"),
            };

            let instruction_type = InstructionType::from(instruction_id);
            match instruction_type {
                InstructionType::NOP => {
                    Bot::op_nop(idx, bots);
                }
                InstructionType::MOV => {
                    Bot::op_mov(idx, op1, op2, bots);
                }
                InstructionType::PUSH => {
                    Bot::op_push(idx, op1, bots);
                },
                InstructionType::POP => {
                    Bot::op_pop(idx, op1, bots);
                },
                InstructionType::CALL => {
                    Bot::op_call(idx, op1, bots);
                },
                InstructionType::RET => {
                    Bot::op_ret(idx, bots);
                },
                InstructionType::JMP => {
                    Bot::op_jmp(idx, op1, bots);
                }
                InstructionType::JL => {
                    Bot::op_jl(idx, op1, bots);
                }
                InstructionType::JLE => {
                    Bot::op_jle(idx, op1, bots);
                }
                InstructionType::JG => {
                    Bot::op_jg(idx, op1, bots);
                }
                InstructionType::JGE => {
                    Bot::op_jge(idx, op1, bots);
                }
                InstructionType::JE => {
                    Bot::op_je(idx, op1, bots);
                }
                InstructionType::JNE => {
                    Bot::op_jne(idx, op1, bots);
                }
                InstructionType::JS => {
                    Bot::op_js(idx, op1, bots);
                }
                InstructionType::JNS => {
                    Bot::op_jns(idx, op1, bots);
                }
                InstructionType::ADD => {
                    simple_math_instr!(idx, op1, op2, bots, +);
                }
                InstructionType::SUB => {
                    simple_math_instr!(idx, op1, op2, bots, -);
                }
                InstructionType::MULT => {
                    simple_math_instr!(idx, op1, op2, bots, *);
                }
                InstructionType::DIV => {
                    Bot::op_div(idx, op1, op2, bots);
                },
                InstructionType::MOD => {
                    Bot::op_mod(idx, op1, op2, bots);
                },
                InstructionType::AND => {
                    simple_math_instr!(idx, op1, op2, bots, &);
                }
                InstructionType::OR => {
                    simple_math_instr!(idx, op1, op2, bots, |);
                }
                InstructionType::XOR => {
                    simple_math_instr!(idx, op1, op2, bots, ^);
                }
                InstructionType::CMP => {
                    Bot::op_cmp(idx, op1, op2, bots);
                }
                InstructionType::TEST => {
                    Bot::op_test(idx, op1, op2, bots);
                },
                InstructionType::GETXY => {
                    Bot::op_getxy(idx, op1, op2, bots);
                },
                InstructionType::ENERGY => {
                    Bot::op_energy(idx, op1, bots);
                },
                InstructionType::TRAVEL => {
                    Bot::op_travel(idx, op1, tank, bots);
                }
                InstructionType::SHL => {
                    Bot::op_shl(idx, op1, op2, bots);
                },
                InstructionType::SHR => {
                    Bot::op_shl(idx, op1, op2, bots);
                },
                InstructionType::SENSE => {
                    Bot::op_sense(idx, op1, tank, bots);
                },
                InstructionType::EAT => {
                    Bot::op_eat(idx, tank, rng, bots);
                },
                InstructionType::RAND => {
                    Bot::op_rand(idx, op1, op2, rng, bots);
                }
                InstructionType::RELEASE => {
                    Bot::op_release(idx, op1, tank, bots);
                },
                InstructionType::CHARGE => {
                    Bot::op_charge(idx, op1, op2, tank, bots);
                },
                InstructionType::POKE => {
                    Bot::op_poke(idx, op1, op2, tank, bots);
                },
                InstructionType::PEEK => {
                    Bot::op_peek(idx, op1, op2, tank, bots);
                },
                // InstructionType::CKSUM => {},
                // not an instruction, do nothing
                _ => Bot::op_nop(idx, bots)
            }
        } else {
            return;
        }
    }
}

// Bot Instructions
impl Bot {
    fn op_nop(idx: usize, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_mov(idx: usize, to: Operand, from: Operand, bots: &mut Vec<Bot>) {
        let data = bots[idx].get(&from);
        bots[idx].put(&to, data);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_push(idx: usize, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&src);
        bots[idx].push(value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_pop(idx: usize, dest: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].pop();
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_call(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;
        let next_ip = bots[idx].instruction_pointer + 3;
        bots[idx].push(next_ip);
        bots[idx].jump_to(to);
    }

    fn op_ret(idx: usize, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;
        let address = bots[idx].pop();
        bots[idx].jump_to(Operand::Direct(Value::Number(address)));
    }

    fn op_jmp(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;
        bots[idx].jump_to(to);
    }

    fn op_jl(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.less {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }

    }

    fn op_jle(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.less || bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_jg(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.greater {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_jge(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.greater || bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_je(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_jne(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if !bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_js(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if bots[idx].flags.success {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_jns(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].energy -= 1;

        if !bots[idx].flags.success {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
    }

    fn op_div(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let divisor = bots[idx].get(&src);
        if divisor == 0 {
            Bot::op_nop(idx, bots);
            return;
        }
        let value = bots[idx].get(&dest) / divisor;
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_mod(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let divisor = bots[idx].get(&src);
        if divisor == 0 {
            Bot::op_nop(idx, bots);
            return;
        }
        let value = bots[idx].get(&dest) % divisor;
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_cmp(idx: usize, op1: Operand, op2: Operand, bots: &mut Vec<Bot>) {
        let lhs = bots[idx].get(&op1);
        let rhs = bots[idx].get(&op2);

        bots[idx].flags.clear();

        if lhs < rhs {
            bots[idx].flags.less = true;
        } else if lhs > rhs {
            bots[idx].flags.greater = true;
        }

        if lhs == rhs {
            bots[idx].flags.equal = true;
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_test(idx: usize, op1: Operand, op2: Operand, bots: &mut Vec<Bot>) {
        let lhs = bots[idx].get(&op1);
        let rhs = bots[idx].get(&op2);

        bots[idx].flags.clear();

        if lhs & rhs == 0 {
            bots[idx].flags.equal = true;
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_getxy(idx: usize, destx: Operand, desty: Operand, bots: &mut Vec<Bot>) {
        let pos = bots[idx].position;
        bots[idx].put(&destx, pos.x as u16);
        bots[idx].put(&desty, pos.y as u16);

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_energy(idx: usize, dest: Operand, bots: &mut Vec<Bot>) {
        let energy = bots[idx].energy;
        bots[idx].put(&dest, energy);

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_travel(idx: usize, direction: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let mut new_position = bots[idx].position.clone();
        let mut failed: bool = false;

        let direction = bots[idx].get(&direction);

        // TODO: break direction check out into a function, for charge/peek/poke
        match direction % 4 {
            0 => {
                if new_position.y == 0 {
                    failed = true;
                } else {
                    new_position.y -= 1;
                }
            }
            1 => {
                if new_position.y == tank.bounds.y - 1 {
                    failed = true;
                } else {
                    new_position.y += 1;
                }
            }
            2 => {
                if new_position.x == tank.bounds.x - 1 {
                    failed = true;
                } else {
                    new_position.x += 1;
                }
            }
            3 => {
                if new_position.x == 0 {
                    failed = true;
                } else {
                    new_position.x -= 1;
                }
            }
            _ => panic!("Travel direction exceeded range ({direction})"),
        };

        if !Bot::is_occupied(&new_position, bots) && !failed && bots[idx].has_energy(10) {
            bots[idx].energy -= 10;
            bots[idx].position = new_position;
        } else if bots[idx].has_energy(1) {
            bots[idx].energy -= 1;
            failed = true;
        } else {
            failed = true;
        }

        bots[idx].flags.success = !failed;

        bots[idx].increment_ip();
    }

    fn op_shl(idx: usize, dest: Operand, amount: Operand, bots: &mut Vec<Bot>) {
        let mut result = bots[idx].get(&dest);

        result = result.wrapping_shl(bots[idx].get(&amount) as u32);

        bots[idx].put(&dest, result);

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_shr(idx: usize, dest: Operand, amount: Operand, bots: &mut Vec<Bot>) {
        let mut result = bots[idx].get(&dest);

        result = result.wrapping_shr(bots[idx].get(&amount) as u32);

        bots[idx].put(&dest, result);

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_sense(idx: usize, dest: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let pos = &bots[idx].position;
        let tile = tank.get_item(pos);
        match tile {
            Some(tile) => {
                bots[idx].put(&dest, tile.id);
                bots[idx].flags.success = true;
            },
            None => {
                bots[idx].put(&dest, 0);
                bots[idx].flags.success = false;
            }
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_eat(idx: usize, tank: &mut Tank, rng: &mut Box<dyn RNGSystem>, bots: &mut Vec<Bot>) {
        let pos = &bots[idx].position;
        let current_energy = bots[idx].energy;

        if current_energy + 2000 > 0xFFFF {
            bots[idx].flags.success = false;
        } else {
            let tile = tank.get_item(pos);
            match tile {
                Some(tile) => {
                    match tile.item_type {
                        ItemType::Sludge => {
                            if tank.eat_item(pos) {
                                bots[idx].mutate(rng);
                            }
                            bots[idx].flags.success = true;
                            bots[idx].energy += 2000;
                        }
                        _ => bots[idx].flags.success = false
                    }
                },
                None => {
                    bots[idx].flags.success = false;
                }
            }
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_rand(idx: usize, to: Operand, max: Operand,
               rng: &mut Box<dyn RNGSystem>, bots: &mut Vec<Bot>
    ) {
        let max = bots[idx].get(&max);
        let result = rng.rand(Some(max as u32)) as u16;

        bots[idx].put(&to, result);

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_release(idx: usize, amount: Operand, tank: &mut Tank, bots: &mut Vec<Bot>) {
        let pos = bots[idx].position.clone();
        let current_energy = bots[idx].energy;
        let amount = bots[idx].get(&amount);

        if current_energy + 1 < amount {
            bots[idx].flags.success = false;
        } else {
            bots[idx].energy -= amount;
            bots[idx].flags.success = tank.deposit(amount, &pos);
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_charge(idx: usize, direction: Operand, amount: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let pos = bots[idx].position.clone();
        let current_energy = bots[idx].energy;

        let amount = bots[idx].get(&amount);

        if current_energy + 1 < amount {
            bots[idx].flags.success = false;
        } else {
            let direction = bots[idx].get(&direction);
            let mut new_position = bots[idx].position.clone();

            match direction % 4 {
                0 => {
                    if new_position.y == 0 {
                        bots[idx].flags.success = false;
                    } else {
                        new_position.y -= 1;
                    }
                }
                1 => {
                    if new_position.y == tank.bounds.y - 1 {
                        bots[idx].flags.success = false;
                    } else {
                        new_position.y += 1;
                    }
                }
                2 => {
                    if new_position.x == tank.bounds.x - 1 {
                        bots[idx].flags.success = false;
                    } else {
                        new_position.x += 1;
                    }
                }
                3 => {
                    if new_position.x == 0 {
                        bots[idx].flags.success = false;
                    } else {
                        new_position.x -= 1;
                    }
                }
                _ => bots[idx].flags.success = false,
            };

            if pos != new_position {
                let other_bot_idx = Bot::occupied_by(&new_position, bots) as usize;
                if other_bot_idx != 0xFFFF {
                    let other_bot_energy = bots[other_bot_idx].energy;

                    if other_bot_energy + amount > 0xFFFF {
                        bots[idx].flags.success = false;
                    } else {
                        bots[idx].energy -= amount;
                        bots[other_bot_idx].energy += amount;
                        bots[idx].flags.success = true;
                    }
                }
            } else {
                bots[idx].flags.success = false;
            }
        }

        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn op_poke(idx: usize, direction: Operand, offset: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        todo!()
    }

    fn op_peek(idx: usize, dest: Operand, amount: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        todo!()
    }

    // Extended Instruction Set (WIP)

    fn op_getid(idx: usize, dest: Operand, bots: &mut Vec<Bot>) {
        let id = bots[idx].id;
        bots[idx].put(&dest, id);
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8, // depth
}

impl PartialEq<Position> for &Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x
        && self.y == other.y
        && self.z == other.z
    }
}

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        Vec2 {
            x: pos.x as i32,
            y: pos.y as i32,
        }
    }
}

impl Position {
    pub fn new(x: u8, y: u8, z: u8) -> Position {
        Position { x, y, z }
    }
}

pub struct Emulator {
    pub rng: Box<dyn RNGSystem>,
    pub tank: Tank,
    pub bots: Vec<Bot>,
    pub iterations: u32,
    pub current_tick: u32,
}

impl Emulator {
    pub fn new(bytecode: &Vec<u16>, iterations: u32, seed: u32, modern_rng: bool) -> Emulator {
        let mut emulator = Emulator {
            rng: match modern_rng {
                true => Box::new(ModernRNG::new(seed)),
                false => Box::new(LegacyRNG::new(seed)),
            },
            tank: Tank::new(Position::new(70, 40, 1)),
            bots: vec![],
            iterations,
            current_tick: 0,
        };

        emulator.tank.initial_fill(200, &mut emulator.rng);

        emulator.bots = Self::create_bots(bytecode, &emulator.tank, &mut emulator.rng);

        emulator
    }

    pub fn create_bots(bytecode: &Vec<u16>, tank: &Tank, rng: &mut Box<dyn RNGSystem>) -> Vec<Bot> {
        let mut bots: Vec<Bot> = vec![];

        for id in 1..=50 {
            let pos: Position = loop {
                let pos = Position {
                    x: rng.rand(Some((tank.bounds.x - 1) as u32)) as u8,
                    y: rng.rand(Some((tank.bounds.y - 1) as u32)) as u8,
                    z: rng.rand(Some((tank.bounds.z - 1) as u32)) as u8,
                };

                if !Bot::is_occupied(&pos, &bots) {
                    break pos;
                }
            };

            let mut bot = Bot::new(id, pos);
            bot.flash(bytecode.clone());
            bots.push(bot);
        }

        for id in 1..=20 {
            let pos: Position = loop {
                let pos = Position {
                    x: rng.rand(Some((tank.bounds.x - 1) as u32)) as u8,
                    y: rng.rand(Some((tank.bounds.y - 1) as u32)) as u8,
                    z: rng.rand(Some((tank.bounds.z - 1) as u32)) as u8,
                };

                if !Bot::is_occupied(&pos, &bots) {
                    break pos;
                }
            };

            let mut bot = Bot::new(id | 0x100, pos);
            bot.flash_drone();
            bots.push(bot);
        }

        bots
    }

    pub fn tick(&mut self) {
        for bot_idx in 0..self.bots.len() {
            Bot::tick(bot_idx, &mut self.tank, &mut self.bots, &mut self.rng);
        }
        self.current_tick += 1;
    }
}
