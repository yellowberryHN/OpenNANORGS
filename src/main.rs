#![allow(dead_code)]

mod cli;
mod compiler;
mod interpreter;
mod parser;
mod rng;
mod symbol_table;
mod tokenizer;

use crate::cli::Arguments;
use crate::compiler::Compiler;
use crate::parser::{Parser, ParserToken};
use crate::symbol_table::SymbolTable;
use crate::tokenizer::Tokenizer;
use byteorder::{BigEndian, WriteBytesExt};
use clap::Parser as clapParse;
use std::fs;
use std::fs::File;
use std::io::Write;

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
        println!("nothing happens here yet.")
    }
}
