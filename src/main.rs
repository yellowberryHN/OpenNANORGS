#![allow(dead_code)]

mod cli;
mod compiler;
mod parser;
mod rng;
mod symbol_table;
mod tokenizer;

use crate::cli::Arguments;
use crate::compiler::Compiler;
use crate::parser::{Parser, ParserToken};
use crate::rng::RNGSystem;
use crate::rng::{LegacyRNG, ModernRNG};
use crate::symbol_table::SymbolTable;
use crate::tokenizer::Tokenizer;
use byteorder::{BigEndian, WriteBytesExt};
use clap::Parser as clapParse;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
enum EntityType {
    Item,
    Bot,
}

#[derive(Debug)]
enum ItemType {
    Sludge,
    CollectionPoint,
    Ramp,
}

#[derive(Debug)]
struct Item {
    id: u16,
    position: Position,
    item_type: ItemType,
}

impl Item {
    fn get_glyph(&self) -> char {
        match self.item_type {
            ItemType::Sludge => '*',
            ItemType::CollectionPoint => '$',
            ItemType::Ramp => '/',
        }
    }
}

#[derive(Debug)]
struct Tank {
    bots: Vec<Bot>,
    bounds: Position,
    score: u64,

    rng: Box<dyn RNGSystem>,

    sludge_amount: u8,
    toxic_sludge: Vec<u8>,

    elements: Vec<Option<Item>>,
}

impl Tank {
    fn new(bounds: Position, seed: u32, use_modern_rng: bool) -> Tank {
        let mut tank = Tank {
            bots: vec![],
            score: 0,
            sludge_amount: 0,
            toxic_sludge: vec![],
            elements: vec![],
            bounds,
            rng: match use_modern_rng {
                true => Box::new(ModernRNG::new(seed)),
                false => Box::new(LegacyRNG::new(seed)),
            },
        };
        tank.elements.resize_with(
            usize::from(tank.bounds.x) * usize::from(tank.bounds.y) * usize::from(tank.bounds.z),
            Default::default,
        );
        tank.bots = Self::create_bots(&mut tank);
        tank
    }

    fn create_bots(&mut self) -> Vec<Bot> {
        let mut bots: Vec<Bot> = vec![];
        for n in 'A'..='Z' {
            bots.push(Bot::new(
                n.to_string(),
                n,
                self.get_random_position(&EntityType::Bot),
            ));
        }
        for n in 'a'..='x' {
            bots.push(Bot::new(
                n.to_string(),
                n,
                self.get_random_position(&EntityType::Bot),
            ));
        }
        bots
    }

    fn get_index(&self, pos: &Position) -> usize {
        println!("checking index of {:?}", pos);
        usize::from(pos.x)
            + usize::from(pos.y) * usize::from(self.bounds.x)
            + usize::from(pos.z) * usize::from(self.bounds.x) * usize::from(self.bounds.y)
    }

    fn is_occupied(&self, pos: &Position, entity: &EntityType) -> bool {
        let index = self.get_index(pos);
        match entity {
            EntityType::Item => self.elements[index].is_some(),
            EntityType::Bot => {
                for bot in &self.bots {
                    if bot.position == *pos {
                        return true;
                    }
                }
                false
            }
        }
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

    fn get_random_position(&mut self, entity: &EntityType) -> Position {
        loop {
            let pos = Position {
                x: u8::try_from(self.rng.rand(Some((self.bounds.x - 1) as u32)))
                    .ok()
                    .unwrap(),
                y: u8::try_from(self.rng.rand(Some((self.bounds.y - 1) as u32)))
                    .ok()
                    .unwrap(),
                z: u8::try_from(self.rng.rand(Some((self.bounds.z - 1) as u32)))
                    .ok()
                    .unwrap(),
            };
            println!("{:?}", pos);
            if !self.is_occupied(&pos, entity) {
                return pos;
            }
        }
    }

    fn fill_with_items(&mut self, num_items: usize) {
        for _ in 0..num_items {
            let pos = self.get_random_position(&EntityType::Item);
            let rand_id = self.rng.rand(Some(self.sludge_amount as u32)) as u16;
            self.add_item(ItemType::Sludge, rand_id, pos);
        }
    }

    fn get_item(&self, pos: &Position) -> Option<&Item> {
        let index = self.get_index(pos);
        self.elements[index].as_ref()
    }

    fn test_random(&mut self, times: u32) {
        for _ in 0..times {
            println!("{}", &self.rng.rand(None));
        }
    }

    fn print(&self, depth: u8) {
        let mut grid = vec![vec![' '; self.bounds.x as usize]; self.bounds.y as usize];

        for i in &self.elements {
            match i {
                None => {}
                Some(item) => {
                    if item.position.x < self.bounds.x && item.position.y < self.bounds.y {
                        grid[item.position.y as usize][item.position.x as usize] = item.get_glyph();
                    }
                }
            }
        }

        for bot in &self.bots {
            if bot.position.x < self.bounds.x && bot.position.y < self.bounds.y {
                grid[bot.position.y as usize][bot.position.x as usize] = bot.glyph;
            }
        }

        for row in grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Bot {
    name: String,
    glyph: char,
    //cpu: CPU;
    position: Position,
    energy: u16,
    sleeping: bool,
}

impl Bot {
    fn new(name: String, glyph: char, position: Position) -> Bot {
        Bot {
            name,
            glyph,
            position,
            energy: 10000,
            sleeping: false,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Position {
    x: u8,
    y: u8,
    z: u8, // depth
}

impl Position {
    fn new(x: u8, y: u8, z: u8) {
        Position { x, y, z };
    }
}

fn render_stuff() {
    let mut test = Bot::new("hhh".to_string(), 'h', Position { x: 0, y: 4, z: 0 });
    println!("{test:?}");
    let mut test2 = Tank::new(Position { x: 70, y: 40, z: 1 }, 69420, false);
    test2.fill_with_items(200);
    println!("{test2:?}");
    println!("there are {} bots", test2.bots.len());
    test2.print(0);
    //test2.test_random(30);
    //println!("Hello, world!");
}

fn main() {
    let args = Arguments::parse();

    let input: String = fs::read_to_string(&args.bot_path).unwrap().parse().unwrap();

    let mut tokenizer = Tokenizer::new(input.clone());

    let tokens = tokenizer.tokenize();

    if args.verbose {
        for token in tokens.clone() {
            println!("{:?}", token);
        }

        println!("-------------------------------------------");
    }

    let mut parser = Parser::new(tokens);
    let mut parser_tokens = Vec::new();

    loop {
        let token = parser.next_token();

        parser_tokens.push(token.clone());

        if token == ParserToken::EOF {
            break;
        }
    }

    if args.verbose {
        for token in parser_tokens.clone() {
            println!("{:#?}", token);
        }
    }

    let symbol_table = SymbolTable::new(&parser_tokens);

    if args.verbose {
        println!("{:#?}", symbol_table.label_to_address);
    }

    let mut compiler = Compiler::new(parser_tokens, symbol_table.label_to_address);
    compiler.compile();

    if args.verbose {
        println!("{:?}", compiler.output);

        let mut bruh = 0;
        for word in &compiler.output {
            print!("{:04x} ", word);
            bruh += 1;
            if bruh == 3 {
                bruh = 0;
                print!("\n");
            }
        }
        if bruh != 3 {
            print!("\n");
        }
    }

    if args.dump_bytecode {
        let file_path = format!("{}.bin", &args.bot_path.display());
        let mut bytecode: File = File::create(&file_path).unwrap();

        for value in &compiler.output {
            bytecode.write_u16::<BigEndian>(*value).unwrap();
        }

        bytecode.flush().unwrap();
        println!("saved to {}", &file_path)
    } else if args.dump_bytecode_text {
        use std::fmt::Write;

        let file_path = format!("{}.txt", &args.bot_path.display());
        let mut output = String::new();

        let mut bruh = 0;
        for word in &compiler.output {
            write!(&mut output, "{:04x} ", word).unwrap();
            bruh += 1;
            if bruh == 3 {
                bruh = 0;
                write!(&mut output, "\n").unwrap();
            }
        }
        if bruh != 3 {
            write!(&mut output, "\n").unwrap();
        }

        fs::write(&file_path, output).unwrap();

        println!("saved to {}", &file_path)
    } else if args.debug_bot.is_some() {
        let bot_char: char = args.debug_bot.unwrap();
        // TODO: do this validation with clap instead
        match bot_char {
            'A'..='Z' | 'a'..='x' => {
                println!("you asked to debug \"{}\"", &bot_char)
            }
            _ => {
                println!("invalid bot identifier \"{}\"", &bot_char)
            }
        }
    } else {
        println!("nothing happens here yet.")
    }
}
