use crate::parser::{Operand, PlusMinus, Register, Value};
use crate::rng::{LegacyRNG, ModernRNG, RNGSystem};
use crate::tokenizer::InstructionType;
use ruscii::spatial::Vec2;
use std::cmp::PartialEq;
use std::collections::HashSet;
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

    /// percentage of sludge density
    pub sludge_density: u8,

    pub sludge_types: u8,
    pub toxic_sludge: Vec<u8>,
    pub elements: Vec<Option<Item>>,

    pub feature_level: FeatureLevel
}

impl Tank {
    pub fn new(bounds: Position, density: u8, feature_level: FeatureLevel) -> Tank {
        let mut tank = Tank {
            score: 0,
            sludge_types: 0,
            toxic_sludge: vec![],
            elements: vec![],
            bounds,
            sludge_density: density,
            feature_level
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
            + (usize::from(pos.y) * usize::from(self.bounds.x))
            + (usize::from(pos.z) * usize::from(self.bounds.x) * usize::from(self.bounds.y))
    }

    pub fn has_item(&self, pos: &Position) -> bool {
        let index = self.get_index(pos);
        self.elements[index].is_some()
    }

    fn add_item(&mut self, item_type: ItemType, id: u16, pos: &Position) {
        let index = self.get_index(pos);
        self.elements[index] = Some(Item {
            id,
            position: *pos,
            item_type,
        });
    }

    fn eat_item(&mut self, idx: usize, pos: &Position, rng: &mut Box<dyn RNGSystem>, bots: &mut Vec<Bot>) -> bool {
        let index = self.get_index(pos);
        match &self.elements[index] {
            Some(item) => {
                match item.item_type {
                    ItemType::Sludge => {
                        let sludge_id = item.id as u8;
                        if self.toxic_sludge.contains(&sludge_id) && bots[idx].id < 50 {
                            bots[idx].mutate(rng);
                        }

                        self.elements[index] = None;
                        self.place_random_sludge(sludge_id, rng);
                        true
                    },
                    _ => false
                }
            },
            None => false
        }
    }

    fn place_random_sludge(&mut self, id: u8, rng: &mut Box<dyn RNGSystem>) {
        let new_pos = self.get_random_position(rng);

        self.add_item(ItemType::Sludge, id as u16, &new_pos);
    }

    pub fn get_random_position(&self, rng: &mut Box<dyn RNGSystem>) -> Position {
        loop {
            let pos = Position {
                x: rng.rand(Some(self.bounds.x as u32)) as u8,
                y: rng.rand(Some(self.bounds.y as u32)) as u8,
                z: match self.feature_level {
                    FeatureLevel::Classic => 0,
                    FeatureLevel::Extended => rng.rand(Some(self.bounds.z as u32)) as u8
                },
            };

            if !self.has_item(&pos) {
                return pos;
            }
        }
    }

    fn calculate_toxic(amount: u8, rng: &mut Box<dyn RNGSystem>) -> Vec<u8> {
        // 20 percent of sludge is toxic
        let toxic_count = ((amount as u32 * 20) / 100) as usize;

        let mut toxic = HashSet::new();

        while toxic.len() < toxic_count {
            // has a chance to roll zero, which means there will technically be one less toxic type
            // this is done to match the behavior in the original
            let num = rng.rand(Some(amount as u32)) as u8;
            toxic.insert(num);
        }

        toxic.into_iter().collect()
    }

    pub fn initial_fill(&mut self, rng: &mut Box<dyn RNGSystem>) {
        self.sludge_types = rng.rand(Some(32)) as u8;
        if self.sludge_types < 5 { self.sludge_types = 5 }

        for _ in 0..10 {
            let pos = self.get_random_position(rng);
            self.add_item(ItemType::CollectionPoint, 0xFFFF, &pos);
        }

        for z in 0..self.bounds.z {
            for y in 0..self.bounds.y {
                for x in 0..self.bounds.x {
                    let pos = Position::new(x,y,z);
                    if rng.rand(Some(100)) < self.sludge_density as u32 && self.get_item(&pos).is_none()  {
                        let rand_id = rng.rand(Some(self.sludge_types as u32)) as u16;
                        self.add_item(ItemType::Sludge, rand_id + 1, &pos);
                    }
                }
            }
        }

        self.toxic_sludge = Tank::calculate_toxic(self.sludge_types, rng);
    }

    fn get_item(&self, pos: &Position) -> Option<&Item> {
        let index = self.get_index(pos);
        self.elements[index].as_ref()
    }

    pub fn generate_power(&mut self, amount: u16, pos: &Position) -> bool {
        match self.get_item(pos) {
            Some(item) => match item.item_type {
                ItemType::CollectionPoint => {
                    self.score += amount as u64;
                    return true;
                }
                _ => false,
            },
            None => false,
        }
    }

    pub fn check_direction(&self, dir: u16, pos: &mut Position) -> bool {
        match dir % 4 {
            0 => {
                if pos.y == 0 {
                    false
                } else {
                    pos.y -= 1;
                    true
                }
            }
            1 => {
                if pos.y == self.bounds.y - 1 {
                    false
                } else {
                    pos.y += 1;
                    true
                }
            }
            2 => {
                if pos.x == self.bounds.x - 1 {
                    false
                } else {
                    pos.x += 1;
                    true
                }
            }
            3 => {
                if pos.x == 0 {
                    false
                } else {
                    pos.x -= 1;
                    true
                }
            }
            _ => panic!("Travel direction exceeded range ({dir})"),
        }
    }
}

#[derive(Debug)]
pub struct Bot {
    pub name: String,
    pub id: u16,
    pub position: Position,
    pub energy: u16,

    // CPU related
    pub instruction_pointer: u16,
    pub stack_pointer: u16,
    pub registers: [u16; 14],
    pub program_memory: [u16; 3600],
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
    ($idx:expr, $dest:expr, $src:expr, $bots:expr, $op:tt) => {
        {
            let idx: usize = $idx;
            let src: Operand = $src;
            let dest: Operand = $dest;
            let bots: &mut Vec<Bot> = $bots;

            let value: u16 = match stringify!($op) {
                "+" => bots[idx].get(&dest).wrapping_add(bots[idx].get(&src)),
                "-" => bots[idx].get(&dest).wrapping_sub(bots[idx].get(&src)),
                "*" => bots[idx].get(&dest).wrapping_mul(bots[idx].get(&src)),
                _ => bots[idx].get(&dest) $op bots[idx].get(&src)
            };

            //let value = bots[idx].get(&dest) $op bots[idx].get(&src);
            bots[idx].put(&dest, value);
        }
    };
}

// Bot Helpers
impl Bot {
    pub fn new(id: u16, position: Position) -> Bot {
        Bot {
            name: "temp".to_string().to_uppercase(),
            id,
            position,
            energy: 10000,

            instruction_pointer: 0,
            stack_pointer: 3600,
            registers: [0u16; 14],
            program_memory: [0u16; 3600],
            flags: CPUFlags::new(),
        }
    }

    pub fn is_active(&self) -> bool { self.energy > 0 }

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

    pub fn get_glyph(&self, inactive: bool) -> char {
        if self.is_active() || !inactive {
            match self.id {
                0..=25 => ((self.id + 65) as u8).into(),
                26..=50 => ((self.id + 71) as u8).into(),
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
            65..=90 => (glyph as u8 - 65).into(),
            97..=120 => (glyph as u8 - 71).into(),
            _ => 0xFFFFu16,
        }
    }

    pub fn has_energy(&self, amount: u16) -> bool {
        self.energy >= amount
    }

    pub fn is_occupied(pos: &Position, bots: &Vec<Bot>) -> bool {
        Self::occupied_by(pos, bots) != 0xFFFF
    }

    pub fn occupied_by(pos: &Position, bots: &Vec<Bot>) -> u16 {
        for bot in bots {
            if bot.position == *pos {
                return bot.id - 1;
            }
        }
        0xFFFFu16
    }

    pub fn travel(idx: usize, dir: u16, tank: &Tank, bots: &mut Vec<Bot>) -> bool {
        let mut new_position = bots[idx].position.clone();
        let in_bounds: bool = tank.check_direction(dir, &mut new_position);

        if in_bounds && !Bot::is_occupied(&new_position, bots) && bots[idx].has_energy(10) {
            bots[idx].energy -= 9;
            bots[idx].position = new_position;
            return true
        }

        false
    }
}

// Bot CPU
impl Bot {
    fn set_instruction_pointer(&mut self, ip: u16) {
        self.instruction_pointer = ip % (self.program_memory.len() as u16 - 3);
        self.instruction_pointer -= self.instruction_pointer % 3;
        //eprintln!("IP: {:?}", self.instruction_pointer);
    }

    fn increment_ip(&mut self) {
        self.set_instruction_pointer(self.instruction_pointer + 3)
    }

    pub fn get_instruction(&self) -> [u16; 3] {
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
            Operand::RegisterIndexedDirect(base, operator, offset) => {
                // TODO: this is horrible
                let register_value = self.get(base.as_ref());
                let offset_value = self.get(offset.as_ref());

                self.get(
                    &Operand::Direct(
                        Value::Number(match operator {
                            PlusMinus::Plus => register_value.wrapping_add(offset_value),
                            PlusMinus::Minus => register_value.wrapping_sub(offset_value)
                        })
                    )
                )
            },
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
            Operand::ImmediateValue(_) => {
                // eprintln!("{}", self.stack_pointer);
                panic!("Attempt to put something into immediate value");
            }
            Operand::RegisterIndexedDirect(base, operator, offset) => {
                let register_value = self.get(base.as_ref());
                let offset_value = self.get(offset.as_ref());

                self.put(
                    &Operand::Direct(
                        Value::Number(match operator {
                            PlusMinus::Plus => register_value.wrapping_add(offset_value),
                            PlusMinus::Minus => register_value.wrapping_sub(offset_value)
                        })
                    ),
                    value
                )
            },
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
        let index = rng.rand(Some(3600)) as usize;
        let value = rng.rand(Some(0x1_0000)) as u16;
        self.program_memory[index] ^= value;
    }

    fn push(&mut self, value: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        if self.stack_pointer >= 3600 {
            self.stack_pointer = 3599;
        }

        self.program_memory[self.stack_pointer as usize] = value;
    }

    fn pop(&mut self) -> u16 {
        let result = if self.stack_pointer < 3600 {
            self.program_memory[self.stack_pointer as usize]
        } else {
            self.stack_pointer = 3599;
            0 // nothing on stack returns 0 always
        };

        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        result
    }

    pub fn tick(idx: usize, tank: &mut Tank, bots: &mut Vec<Bot>, rng: &mut Box<dyn RNGSystem>) {
        let mut increment_ip: bool = true;

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
                    let op1_reg = op1_value >> 12;
                    let op1_reg_offset = op1_value & 0xFFF;

                    let op1_reg_sub = instruction[0] >> 11 & 0x1 == 1;

                    Operand::RegisterIndexedDirect(
                        Box::new(Operand::Register(op1_reg.into())),
                        if op1_reg_sub { PlusMinus::Minus } else { PlusMinus::Plus },
                        Box::new(Operand::ImmediateValue(Value::Number(op1_reg_offset)))
                    )
                }
                _ => panic!("Unknown addressing mode"),
            };

            let op2 = match op2_type {
                0 => Operand::Direct(Value::Number(op2_value)),
                1 => Operand::Register(op2_value.into()),
                2 => Operand::ImmediateValue(Value::Number(op2_value)),
                3 => {
                    let op2_reg = op2_value >> 12;
                    let op2_reg_offset = op2_value & 0xFFF;

                    let op2_reg_sub = instruction[0] >> 10 & 0x1 == 1;

                    Operand::RegisterIndexedDirect(
                        Box::new(Operand::Register(op2_reg.into())),
                        if op2_reg_sub { PlusMinus::Minus } else { PlusMinus::Plus },
                        Box::new(Operand::ImmediateValue(Value::Number(op2_reg_offset)))
                    )
                }
                _ => panic!("Unknown addressing mode"),
            };

            let instruction_type = InstructionType::from(instruction_id);
            // eprintln!("{:#?} - {:?} {:?}", instruction_type, op1, op2);

            match instruction_type {
                InstructionType::MOV => {
                    Bot::op_mov(idx, op1, op2, bots);
                }
                InstructionType::PUSH => {
                    Bot::op_push(idx, op1, bots);
                }
                InstructionType::POP => {
                    Bot::op_pop(idx, op1, bots);
                }
                InstructionType::CALL => {
                    increment_ip = Bot::op_call(idx, op1, bots);
                }
                InstructionType::RET => {
                    increment_ip = Bot::op_ret(idx, bots);
                }
                InstructionType::JMP => {
                    increment_ip = Bot::op_jmp(idx, op1, bots);
                }
                InstructionType::JL => {
                    increment_ip = Bot::op_jl(idx, op1, bots);
                }
                InstructionType::JLE => {
                    increment_ip = Bot::op_jle(idx, op1, bots);
                }
                InstructionType::JG => {
                    increment_ip = Bot::op_jg(idx, op1, bots);
                }
                InstructionType::JGE => {
                    increment_ip = Bot::op_jge(idx, op1, bots);
                }
                InstructionType::JE => {
                    increment_ip = Bot::op_je(idx, op1, bots);
                }
                InstructionType::JNE => {
                    increment_ip = Bot::op_jne(idx, op1, bots);
                }
                InstructionType::JS => {
                    increment_ip = Bot::op_js(idx, op1, bots);
                }
                InstructionType::JNS => {
                    increment_ip = Bot::op_jns(idx, op1, bots);
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
                }
                InstructionType::MOD => {
                    Bot::op_mod(idx, op1, op2, bots);
                }
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
                }
                InstructionType::GETXY => {
                    Bot::op_getxy(idx, op1, op2, bots);
                }
                InstructionType::ENERGY => {
                    Bot::op_energy(idx, op1, bots);
                }
                InstructionType::TRAVEL => {
                    Bot::op_travel(idx, op1, tank, bots);
                }
                InstructionType::SHL => {
                    Bot::op_shl(idx, op1, op2, bots);
                }
                InstructionType::SHR => {
                    Bot::op_shl(idx, op1, op2, bots);
                }
                InstructionType::SENSE => {
                    Bot::op_sense(idx, op1, tank, bots);
                }
                InstructionType::EAT => {
                    Bot::op_eat(idx, tank, rng, bots);
                }
                InstructionType::RAND => {
                    Bot::op_rand(idx, op1, op2, rng, bots);
                }
                InstructionType::RELEASE => {
                    Bot::op_release(idx, op1, tank, bots);
                }
                InstructionType::CHARGE => {
                    Bot::op_charge(idx, op1, op2, tank, bots);
                }
                InstructionType::POKE => {
                    Bot::op_poke(idx, op1, op2, tank, bots);
                }
                InstructionType::PEEK => {
                    Bot::op_peek(idx, op1, op2, tank, bots);
                }
                InstructionType::CKSUM => {
                    Bot::op_cksum(idx, op1, op2, bots);
                },
                // not an instruction, do nothing
                _ => Bot::op_nop()
            };
        }

        bots[idx].energy = bots[idx].energy.saturating_sub(1);
        if increment_ip { bots[idx].increment_ip(); }
    }
}

// Bot Instructions
impl Bot {
    fn op_nop() {
        return // do nothing
    }

    fn op_mov(idx: usize, to: Operand, from: Operand, bots: &mut Vec<Bot>) {
        let data = bots[idx].get(&from);
        bots[idx].put(&to, data);
    }

    fn op_push(idx: usize, src: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].get(&src);
        bots[idx].push(value);
    }

    fn op_pop(idx: usize, dest: Operand, bots: &mut Vec<Bot>) {
        let value = bots[idx].pop();
        bots[idx].put(&dest, value);
    }

    fn op_call(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        let location = bots[idx].instruction_pointer + 3;
        bots[idx].push(location);
        bots[idx].jump_to(to);
        false
    }

    fn op_ret(idx: usize, bots: &mut Vec<Bot>) -> bool {
        let location = bots[idx].pop();
        bots[idx].set_instruction_pointer(location);
        false
    }

    fn op_jmp(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        bots[idx].jump_to(to);
        false
    }

    fn op_jl(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if bots[idx].flags.less {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_jle(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if bots[idx].flags.less || bots[idx].flags.equal {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_jg(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool{
        if bots[idx].flags.greater {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_jge(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if bots[idx].flags.greater || bots[idx].flags.equal {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_je(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if bots[idx].flags.equal {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_jne(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if !bots[idx].flags.equal {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_js(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if bots[idx].flags.success {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_jns(idx: usize, to: Operand, bots: &mut Vec<Bot>) -> bool {
        if !bots[idx].flags.success {
            bots[idx].jump_to(to);
            false
        } else { true }
    }

    fn op_div(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let divisor = bots[idx].get(&src);
        if divisor != 0 {
            let value = bots[idx].get(&dest) / divisor;
            bots[idx].put(&dest, value);
        }
    }

    fn op_mod(idx: usize, dest: Operand, src: Operand, bots: &mut Vec<Bot>) {
        let divisor = bots[idx].get(&src);
        if divisor != 0 {
            let value = bots[idx].get(&dest) % divisor;
            bots[idx].put(&dest, value);
        }
    }

    fn op_cmp(idx: usize, op1: Operand, op2: Operand, bots: &mut Vec<Bot>) {
        let lhs = bots[idx].get(&op1);
        let rhs = bots[idx].get(&op2);

        bots[idx].flags.clear();

        if lhs < rhs {
            bots[idx].flags.less = true;
        } else if lhs > rhs {
            bots[idx].flags.greater = true;
        } else if lhs == rhs {
            bots[idx].flags.equal = true;
        }
    }

    fn op_test(idx: usize, op1: Operand, op2: Operand, bots: &mut Vec<Bot>) {
        let lhs = bots[idx].get(&op1);
        let rhs = bots[idx].get(&op2);

        bots[idx].flags.clear();

        if lhs & rhs == 0 {
            bots[idx].flags.equal = true;
        }
    }

    fn op_getxy(idx: usize, destx: Operand, desty: Operand, bots: &mut Vec<Bot>) {
        let pos = bots[idx].position;
        bots[idx].put(&destx, pos.x as u16);
        bots[idx].put(&desty, pos.y as u16);
    }

    fn op_energy(idx: usize, dest: Operand, bots: &mut Vec<Bot>) {
        let energy = bots[idx].energy;
        bots[idx].put(&dest, energy);
    }

    fn op_travel(idx: usize, direction: Operand, tank: &Tank, bots: &mut Vec<Bot>)  {
        let direction = bots[idx].get(&direction);
        bots[idx].flags.success = Bot::travel(idx, direction, tank, bots);
    }

    fn op_shl(idx: usize, dest: Operand, amount: Operand, bots: &mut Vec<Bot>) {
        let mut result = bots[idx].get(&dest);
        let mut amount = bots[idx].get(&amount);
        if amount > 16 { amount = 16; }

        result = result.wrapping_shl(amount as u32);

        bots[idx].put(&dest, result);
    }

    fn op_shr(idx: usize, dest: Operand, amount: Operand, bots: &mut Vec<Bot>) {
        let mut result = bots[idx].get(&dest);
        let mut amount = bots[idx].get(&amount);
        if amount > 16 { amount = 16; }

        result = result.wrapping_shr(amount as u32);

        bots[idx].put(&dest, result);
    }

    fn op_sense(idx: usize, dest: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let pos = &bots[idx].position;
        let tile = tank.get_item(pos);
        match tile {
            Some(tile) => {
                bots[idx].put(&dest, tile.id);
                bots[idx].flags.success = true;
            }
            None => {
                bots[idx].put(&dest, 0);
                bots[idx].flags.success = false;
            }
        }
    }

    fn op_eat(idx: usize, tank: &mut Tank, rng: &mut Box<dyn RNGSystem>, bots: &mut Vec<Bot>) {
        let pos = bots[idx].position;
        let current_energy = bots[idx].energy;

        if current_energy as u32 + 2000 < 0xFFFF && tank.eat_item(idx, &pos, rng, bots) {
            bots[idx].flags.success = true;
            bots[idx].energy += 2000;
        } else {
            bots[idx].flags.success = false;
        }
    }

    fn op_rand(idx: usize, to: Operand, max: Operand, rng: &mut Box<dyn RNGSystem>, bots: &mut Vec<Bot>) {
        let max = bots[idx].get(&max);
        if max != 0 {
            let result = rng.rand(Some(max as u32)) as u16;
            bots[idx].put(&to, result);
        }
    }

    fn op_release(idx: usize, amount: Operand, tank: &mut Tank, bots: &mut Vec<Bot>) {
        let amount = bots[idx].get(&amount);
        let pos = bots[idx].position;

        if (bots[idx].energy + 1) < amount || amount == 0 {
            bots[idx].flags.success = false;
        } else {
            bots[idx].flags.success = tank.generate_power(amount, &pos);
            bots[idx].energy -= amount;
        }
    }

    fn op_charge(idx: usize, direction: Operand, amount: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let amount = bots[idx].get(&amount);
        let pos = bots[idx].position;

        if bots[idx].energy + 1 < amount  {
            bots[idx].flags.success = false;
        } else {
            let direction = bots[idx].get(&direction);
            let mut new_position = bots[idx].position.clone();

            if !tank.check_direction(direction, &mut new_position) {
                bots[idx].flags.success = false;
            }

            if pos != new_position {
                let other_bot_idx = Bot::occupied_by(&new_position, bots) as usize;
                if other_bot_idx != 0xFFFF {
                    let other_bot_energy = bots[other_bot_idx].energy as u32;

                    if other_bot_energy + amount as u32 > 0xFFFF {
                        bots[idx].flags.success = false;
                    } else {
                        bots[other_bot_idx].energy += amount;
                        bots[idx].energy -= amount;
                        bots[idx].flags.success = true;
                    }
                } else {
                    bots[idx].flags.success = false;
                }
            } else {
                bots[idx].flags.success = false;
            }
        }
    }

    fn op_poke(idx: usize, direction: Operand, offset: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let direction = bots[idx].get(&direction);
        let mut new_position = bots[idx].position.clone();

        if tank.check_direction(direction, &mut new_position) {
            let other_bot_idx = Bot::occupied_by(&new_position, bots) as usize;
            if other_bot_idx != 0xFFFF {
                let offset = bots[idx].get(&offset) as usize;
                if offset < 3600 {
                    bots[other_bot_idx].program_memory[offset] = bots[idx].registers[0];
                    bots[idx].flags.success = true;
                } else {
                    bots[idx].flags.success = false;
                }
            } else {
                bots[idx].flags.success = false;
            }
        } else {
            bots[idx].flags.success = false;
        }
    }

    fn op_peek(idx: usize, dest: Operand, offset: Operand, tank: &Tank, bots: &mut Vec<Bot>) {
        let direction = bots[idx].get(&dest);
        let mut new_position = bots[idx].position.clone();

        if tank.check_direction(direction, &mut new_position) {
            let other_bot_idx = Bot::occupied_by(&new_position, bots) as usize;
            if other_bot_idx != 0xFFFF {
                let offset = bots[idx].get(&offset) as usize;
                if offset < 3600 {
                    let value = bots[other_bot_idx].program_memory[offset];
                    bots[idx].put(&dest, value);
                    bots[idx].flags.success = true;
                } else {
                    bots[idx].flags.success = false;
                }
            } else {
                bots[idx].flags.success = false;
            }
        } else {
            bots[idx].flags.success = false;
        }
    }

    fn op_cksum(idx: usize, start: Operand, end: Operand, bots: &mut Vec<Bot>) {

        let start_idx = bots[idx].get(&start) as usize;
        let end_idx = bots[idx].get(&end) as usize;

        if start_idx < 3600 && end_idx < 3601 && start_idx < end_idx {
            let cksum: u16 = bots[idx].program_memory[start_idx..end_idx].iter().fold(0u16, |acc, &m| acc.wrapping_add(m));
            bots[idx].put(&start, cksum);
        }
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
        self.x == other.x && self.y == other.y && self.z == other.z
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

    pub finished: bool,

    pub feature_level: FeatureLevel
}

#[derive(Copy, Clone, Debug)]
pub enum FeatureLevel {
    Classic,
    Extended
}

impl Emulator {
    pub fn new(bytecode: &Vec<u16>, iterations: u32, seed: u32, feature_level: FeatureLevel, modern_rng: bool) -> Emulator {
        let mut emulator = Emulator {
            rng: match modern_rng {
                true => Box::new(ModernRNG::new(seed)),
                false => Box::new(LegacyRNG::new(seed)),
            },
            tank: Tank::new(Position::new(70, 40, 1), 10, feature_level),
            bots: vec![],
            iterations,
            current_tick: 0,
            finished: false,
            feature_level,
        };

        emulator.tank.initial_fill(&mut emulator.rng);

        emulator.create_bots(bytecode);

        emulator
    }

    fn create_bots(&mut self, bytecode: &Vec<u16>) {
        self.bots = vec![];

        for id in 0..50 {
            let pos: Position = loop {
                let pos = Position {
                    x: self.rng.rand(Some(self.tank.bounds.x as u32)) as u8,
                    y: self.rng.rand(Some(self.tank.bounds.y as u32)) as u8,
                    z: match self.feature_level {
                        FeatureLevel::Classic => 0,
                        FeatureLevel::Extended => self.rng.rand(Some(self.tank.bounds.z as u32)) as u8
                    },
                };

                if !Bot::is_occupied(&pos, &self.bots) {
                    break pos;
                }
            };

            let mut bot = Bot::new(id, pos);
            bot.flash(bytecode.clone());
            self.bots.push(bot);
        }

        for id in 0..20 {
            let pos: Position = loop {
                let pos = Position {
                    x: self.rng.rand(Some(self.tank.bounds.x as u32)) as u8,
                    y: self.rng.rand(Some(self.tank.bounds.y as u32)) as u8,
                    z: match self.feature_level {
                        FeatureLevel::Classic => 0,
                        FeatureLevel::Extended => self.rng.rand(Some(self.tank.bounds.z as u32)) as u8
                    },
                };

                if !Bot::is_occupied(&pos, &self.bots) {
                    break pos;
                }
            };

            let mut bot = Bot::new(id + 50, pos);
            bot.flash_drone();
            self.bots.push(bot);
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut active = 0;

        for bot_idx in 0..self.bots.len() {
            if self.bots[bot_idx].is_active() {
                Bot::tick(bot_idx, &mut self.tank, &mut self.bots, &mut self.rng);
                active += 1;
            }
        }

        self.current_tick += 1;
        self.finished = active == 0 || self.current_tick >= self.iterations;

        !self.finished
    }

    pub fn active_bot_count(&self) -> (u32, u32) {
        let mut bots = 0;
        let mut drones = 0;

        for bot in &self.bots {
            match bot.id {
                1..=50 => {
                    if bot.is_active() {
                        bots += 1
                    }
                }
                _ => {
                    if bot.is_active() {
                        drones += 1
                    }
                }
            }
        }

        (bots, drones)
    }
}
