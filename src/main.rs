#![allow(dead_code)]

mod cli;
mod compiler;
mod cpu;
mod disassembler;
mod emulator;
mod instruction;
mod parser;
mod rng;
mod symbol_table;
pub mod tokenizer;

use crate::cli::Arguments;
use crate::compiler::Compiler;
use crate::disassembler::Disassembler;
use crate::parser::{Parser, ParserToken};
use crate::symbol_table::SymbolTable;
use crate::tokenizer::Tokenizer;
use byteorder::{BigEndian, WriteBytesExt};
use clap::Parser as clapParse;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::emulator::Position;

fn main() {
    let args = Arguments::parse();

    let compiler = Compiler::new_from_file(&args.bot_path, args.verbose);

    if args.show_disassembly {
        let disassembler = Disassembler::new(compiler.output.clone());

        disassembler.print_disassembly(
            (&args.bot_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    } else if args.dump_bytecode {
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

        let mut word_count = 0;

        for word in &compiler.output {
            if word_count == 2 {
                write!(&mut output, "{:04x}", word).unwrap();
            } else {
                write!(&mut output, "{:04x} ", word).unwrap();
            }

            word_count += 1;

            if word_count == 3 {
                word_count = 0;
                write!(&mut output, "\n").unwrap();
            }
        }
        if word_count != 3 {
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
        /*
        let mut test_bot = emulator::Bot::new("hhh".to_string(), 'h', emulator::Position::new(10,4,0));
        test_bot.flash(compiler.output.clone());

        test_bot.tick();
        test_bot.tick();
        test_bot.tick();

        println!("{:?}", test_bot);
        */

        let mut test_tank = crate::emulator::Tank::new(Position { x: 70, y: 40, z: 1 }, 69420, false);
        test_tank.fill_with_items(200);

        for bot in &mut test_tank.bots {
            bot.flash(compiler.output.clone());
        }

        for bot in &mut test_tank.bots {
            bot.tick();
            bot.tick();
            bot.tick();
        }

        println!("{:?}", test_tank);

        // println!("nothing happens here yet.")
    }
}
