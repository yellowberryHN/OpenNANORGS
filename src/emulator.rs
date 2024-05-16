use std::cmp::min;
use ruscii::spatial::Vec2;
use crate::disassembler::Disassembler;
use crate::rng::{LegacyRNG, ModernRNG, RNGSystem};
use crate::tokenizer::InstructionType;

#[derive(Debug)]
enum ItemType {
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
    score: u64,
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

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: u16,
    pub position: Position,
    pub energy: u16,
    sleeping: bool,

    // CPU related
    pub(crate) instruction_pointer: u16,
    stack_pointer: u16,
    registers: [u16; 14],
    program_memory: [u16; 3600],
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
            0x8004, 0x000F, 0x0000,
            0x8004, 0x0018, 0x0000,
            0x8004, 0x002A, 0x0000,
            0x8004, 0x0045, 0x0000,
            0x8006, 0xFFF7, 0x0000,
            0x2020, 0x0DFB, 0x0004,
            0x2020, 0x0DFC, 0x000A,
            0x200F, 0x0DFC, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x401E, 0x0002, 0x0000,
            0x800E, 0x000F, 0x0000,
            0x401A, 0x0002, 0x0000,
            0x6017, 0x0002, 0x2710,
            0x8009, 0x0006, 0x0000,
            0x001F, 0x0000, 0x0000,
            0x0005, 0x0000, 0x0000,
            0x4001, 0x0000, 0x0DFB,
            0x6020, 0x0001, 0x0DF8,
            0x5024, 0x0000, 0x0001,
            0x800E, 0x0012, 0x0000,
            0x7017, 0x0000, 0x1000,
            0x800B, 0x000C, 0x0000,
            0x6020, 0x0001, 0x0E10,
            0x7001, 0x0000, 0x1000,
            0x1023, 0x0DFB, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x2017, 0x0DFC, 0x0000,
            0x800B, 0x000F, 0x0000,
            0x001B, 0x0DFB, 0x0000,
            0x800E, 0x0009, 0x0000,
            0x2010, 0x0DFC, 0x0001,
            0x0005, 0x0000, 0x0000,
            0x8004, 0xFFAF, 0x0000,
            0x8006, 0xFFEB, 0x0000
        ];

        self.flash(malicious);
    }

    pub fn get_glyph(&self) -> char {
        if self.energy > 0 {
            match self.id {
                0..=26 => ((self.id+64) as u8).into(),
                27..=50 => ((self.id+70) as u8).into(),
                _ => '@'
            }
        } else {
            match self.id {
                0..=50 => '.',
                _ => ','
            }
        }
    }

    pub fn id_from_glyph(glyph: char) -> u16 {
        match glyph as u8 {
            64..=90 => (glyph as u8-64).into(),
            97..=120 => (glyph as u8-70).into(),
            _ => 0u16
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

    pub fn tick(idx: usize, tank: &mut Tank, bots: &mut Vec<Bot>, rng: &mut Box<dyn RNGSystem>) {
        // TODO: this is where instructions will be ran
        let instruction = bots[idx].get_instruction();
        let instruction_id = instruction[0] & 0xFF;

        //println!("{}", Disassembler::parse(instruction, bots[idx].instruction_pointer, false));
        if instruction_id <= InstructionType::CKSUM as u16 {
            let instruction_type = InstructionType::from(instruction_id);
            match instruction_type {
                InstructionType::NOP => { bots[idx].increment_ip(); },
                // InstructionType::MOV => {},
                // InstructionType::PUSH => {},
                // InstructionType::POP => {},
                // InstructionType::CALL => {},
                // InstructionType::RET => {},
                // InstructionType::JMP => {},
                // InstructionType::JL => {},
                // InstructionType::JLE => {},
                // InstructionType::JG => {},
                // InstructionType::JGE => {},
                // InstructionType::JE => {},
                // InstructionType::JNE => {},
                // InstructionType::JS => {},
                // InstructionType::JNS => {},
                // InstructionType::ADD => {},
                // InstructionType::SUB => {},
                // InstructionType::MULT => {},
                // InstructionType::DIV => {},
                // InstructionType::MOD => {},
                // InstructionType::AND => {},
                // InstructionType::OR => {},
                // InstructionType::XOR => {},
                // InstructionType::CMP => {},
                // InstructionType::TEST => {},
                // InstructionType::GETXY => {},
                // InstructionType::ENERGY => {},
                // InstructionType::TRAVEL => {},
                // InstructionType::SHL => {},
                // InstructionType::SHR => {},
                // InstructionType::SENSE => {},
                // InstructionType::EAT => {},
                InstructionType::RAND => {
                    // TODO: actually set things
                    rng.rand(None);
                    bots[idx].increment_ip();
                },
                // InstructionType::RELEASE => {},
                // InstructionType::CHARGE => {},
                // InstructionType::POKE => {},
                // InstructionType::PEEK => {},
                // InstructionType::CKSUM => {},
                _ => { // TODO: placeholder, do nothing
                    // Bot::travel(idx, rng.rand(3.into()) as u16, tank, bots);
                    bots[idx].increment_ip();
                }
            }
        }
        else {
            return;
        }
    }
}

// Bot Instructions
impl Bot {
    fn getid(idx: usize, bots: &Vec<Bot>) -> u16 {
        bots[idx].id
    }

    // TODO: should each instruction change flags?
    fn travel(idx: usize, direction: u16, tank: &Tank, bots: &mut Vec<Bot>) -> bool {
        let mut new_position = bots[idx].position.clone();
        let mut failed: bool = false;

        match direction % 4 {
            0 => {
                if new_position.y == 0 {
                    failed = true;
                } else {
                    new_position.y -= 1;
                }
            },
            1 => {
                if new_position.y == 39 {
                    failed = true;
                } else {
                    new_position.y += 1;
                }
            },
            2 => {
                if new_position.x == 69 {
                    failed = true;
                } else {
                    new_position.x += 1;
                }
            },
            3 => {
                if new_position.x == 0 {
                    failed = true;
                } else {
                    new_position.x -= 1;
                }
            },
            _ => panic!("Travel direction exceeded range ({direction})")
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

        failed == false
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
    pub current_tick: u32
}

impl Emulator {
    pub fn new(bytecode: &Vec<u16>, iterations: u32, seed: u32, modern_rng: bool) -> Emulator {
        let mut emulator = Emulator {
            rng: match modern_rng {
                true => Box::new(ModernRNG::new(seed)),
                false => Box::new(LegacyRNG::new(seed)),
            },
            tank: Tank::new(Position::new(70,40,1)),
            bots: vec![],
            iterations,
            current_tick: 0
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
    }
}
