use crate::parser::{Operand, Register, Value};
use crate::rng::{LegacyRNG, ModernRNG, RNGSystem};
use crate::tokenizer::InstructionType;
use ruscii::spatial::Vec2;
use std::fmt::Formatter;

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

    fn remove_item(&mut self, pos: &Position) {
        let index = self.get_index(pos);
        self.elements[index] = None;
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

    /*fn test_random(&mut self, times: u32) {
        for _ in 0..times {
            println!("{}", &self.rng.rand(None));
        }
    }*/
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
                    bots[idx].energy -= 1;
                    bots[idx].increment_ip();
                }
                InstructionType::MOV => {
                    Bot::mov(idx, op1, op2, bots);
                    bots[idx].increment_ip();
                }
                // InstructionType::PUSH => {},
                // InstructionType::POP => {},
                // InstructionType::CALL => {},
                // InstructionType::RET => {},
                InstructionType::JMP => {
                    Bot::jmp(idx, op1, bots);
                }
                InstructionType::JL => {
                    Bot::jl(idx, op1, bots);
                }
                InstructionType::JLE => {
                    Bot::jle(idx, op1, bots);
                }
                InstructionType::JG => {
                    Bot::jg(idx, op1, bots);
                }
                InstructionType::JGE => {
                    Bot::jge(idx, op1, bots);
                }
                InstructionType::JE => {
                    Bot::je(idx, op1, bots);
                }
                InstructionType::JNE => {
                    Bot::jne(idx, op1, bots);
                }
                InstructionType::JS => {
                    Bot::js(idx, op1, bots);
                }
                InstructionType::JNS => {
                    Bot::jns(idx, op1, bots);
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
                // InstructionType::DIV => {},
                // InstructionType::MOD => {},
                InstructionType::AND => {
                    simple_math_instr!(idx, op1, op2, bots, &);
                }
                InstructionType::OR => {
                    simple_math_instr!(idx, op1, op2, bots, |);
                }
                InstructionType::XOR => {
                    simple_math_instr!(idx, op1, op2, bots, ^);
                }
                // InstructionType::CMP => {},
                // InstructionType::TEST => {},
                // InstructionType::GETXY => {},
                // InstructionType::ENERGY => {},
                InstructionType::TRAVEL => {
                    Bot::travel(idx, op1, tank, bots);
                    bots[idx].increment_ip();
                }
                // InstructionType::SHL => {},
                // InstructionType::SHR => {},
                // InstructionType::SENSE => {},
                // InstructionType::EAT => {},
                InstructionType::RAND => {
                    Bot::rand(idx, op1, op2, rng, bots);
                    bots[idx].increment_ip();
                }
                // InstructionType::RELEASE => {},
                // InstructionType::CHARGE => {},
                // InstructionType::POKE => {},
                // InstructionType::PEEK => {},
                // InstructionType::CKSUM => {},
                _ => {
                    // not an instruction, do nothing
                    bots[idx].energy -= 1;
                    bots[idx].increment_ip();
                }
            }
        } else {
            return;
        }
    }
}

// Bot Instructions
impl Bot {
    fn getid(idx: usize, bots: &Vec<Bot>) -> u16 {
        bots[idx].id
    }

    fn mov(idx: usize, to: Operand, from: Operand, bots: &mut Vec<Bot>) {
        let data = bots[idx].get(&from);
        bots[idx].put(&to, data);
        bots[idx].energy -= 1;
    }

    fn jmp(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        bots[idx].jump_to(to);
        bots[idx].energy -= 1;
    }

    fn jl(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.less {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn jle(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.less || bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn jg(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.greater {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn jge(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.greater || bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn je(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn jne(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if !bots[idx].flags.equal {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn js(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if bots[idx].flags.success {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn jns(idx: usize, to: Operand, bots: &mut Vec<Bot>) {
        if !bots[idx].flags.success {
            bots[idx].jump_to(to);
        } else {
            bots[idx].increment_ip();
        }
        bots[idx].energy -= 1;
    }

    fn rand(
        idx: usize,
        to: Operand,
        max: Operand,
        rng: &mut Box<dyn RNGSystem>,
        bots: &mut Vec<Bot>,
    ) {
        let max = bots[idx].get(&max);
        let result = rng.rand(Some(max as u32)) as u16;

        bots[idx].put(&to, result);

        bots[idx].energy -= 1;
    }

    fn travel(idx: usize, direction: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let mut new_position = bots[idx].position.clone();
        let mut failed: bool = false;

        let direction = bots[idx].get(&direction);

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
    }

    fn add(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) + bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn sub(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) - bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn mult(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) * bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    // fn div(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
    //     let value = bots[idx].get(&dest) / bots[idx].get(&src);
    //     bots[idx].put(&dest, value);
    //     bots[idx].energy -= 1;
    //     bots[idx].increment_ip();
    // }
    //

    fn and(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) & bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn or(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) | bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }

    fn xor(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&dest) ^ bots[idx].get(&src);
        bots[idx].put(&dest, value);
        bots[idx].energy -= 1;
        bots[idx].increment_ip();
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8, // depth
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
