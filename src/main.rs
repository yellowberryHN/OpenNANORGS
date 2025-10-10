#![allow(dead_code)]

mod cli;
mod compiler;
mod disassembler;
mod emulator;
mod parser;
mod rng;
mod symbol_table;
pub mod tokenizer;

use crate::cli::Arguments;
use crate::compiler::Compiler;
use crate::disassembler::Disassembler;
use crate::emulator::{Bot, Emulator, ItemType};
use byteorder::{LittleEndian, WriteBytesExt};
use clap::Parser as clapParse;
use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{Instant, SystemTime};

fn main() {
    let mut args = Arguments::parse();

    if args.seed.is_none() {
        args.seed = Some(
            (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % u32::MAX as u64) as u32,
        );
    }



    let bytecode: Vec<u16> = if args.as_bytecode {
        let code_file = match fs::read(&args.bot_path) {
            Ok(data) => data,
            Err(_) => {
                println!("file not found: {}", &args.bot_path.display());
                return;
            }
        };

        let mut pairs: Vec<u16> = code_file.chunks_exact(2).map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())).collect();

        if pairs.len() > 3600 {
            println!("error: bytecode too large! {} words long, should be 3600", pairs.len());
            return;
        }
        pairs.resize(3600, 0);

        pairs
    } else {
        let compiler = Compiler::new_from_file(&args.bot_path, args.verbose);
        compiler.output.clone()
    };

    if args.show_disassembly {
        let disassembler = Disassembler::new(bytecode.clone());

        disassembler.print_disassembly(
            (&args.bot_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        return;
    } else if args.dump_bytecode {
        let file_path = format!("{}.bin", &args.bot_path.display());
        let mut dumped_bytecode: File = File::create(&file_path).unwrap();

        for value in &bytecode {
            dumped_bytecode.write_u16::<LittleEndian>(*value).unwrap();
        }

        dumped_bytecode.flush().unwrap();
        println!("saved to {}", &file_path);
        return;
    } else if args.dump_bytecode_text {
        use std::fmt::Write;

        let file_path = format!("{}.txt", &args.bot_path.display());
        let mut output = String::new();

        let mut word_count = 0;

        for word in &bytecode {
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

        println!("saved to {}", &file_path);
        return;
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
    }

    let mut emulator = Emulator::new(&bytecode, args.iterations, args.seed.unwrap(), args.modern_rng);

    //println!("seed is {}", args.seed.unwrap());

    if args.quiet_mode {
        let now = Instant::now();
        while emulator.current_tick < emulator.iterations {
            if !emulator.tick() { break; }
        }
        println!("done in {}ms", now.elapsed().as_millis())
    } else {
        let mut fps_counter = FPSCounter::default();
        let mut app = App::config(Config::fps(Config::new(), 120));

        app.run(|app_state: &mut State, window: &mut Window| {
            // TODO: this is moderately annoying, figure out how to allow Ctrl+C
            for key_event in app_state.keyboard().last_key_events() {
                match key_event {
                    KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                    KeyEvent::Pressed(Key::Q) => app_state.stop(),
                    _ => (),
                }
            }

            if emulator.current_tick < emulator.iterations {
                if !emulator.tick() { app_state.stop() }
            } else {
                app_state.stop()
            }

            fps_counter.update();

            let mut pencil = Pencil::new(window.canvas_mut());

            for element in &emulator.tank.elements {
                let element = element.as_ref();
                match element {
                    Some(element) => {
                        match element.item_type {
                            ItemType::Sludge => {
                                if emulator.tank.toxic_sludge.contains(&(element.id as u8)) {
                                    pencil.set_foreground(Color::Xterm(28))
                                } else { pencil.set_foreground(Color::Grey) }
                            },
                            ItemType::CollectionPoint => pencil.set_foreground(Color::Xterm(6)),
                            ItemType::Ramp => pencil.set_foreground(Color::DarkGrey),
                        };

                        pencil.draw_char(element.get_glyph(), element.position.into());
                    }
                    None => {}
                }
            }

            let debug_bot_id = match args.debug_bot {
                Some(glyph) => Bot::id_from_glyph(glyph),
                None => 0xFFFF,
            };

            for bot in &emulator.bots {
                if debug_bot_id != 0xFFFF && bot.id == debug_bot_id {
                    pencil.set_foreground(Color::Xterm(208));
                } else if bot.id > 50 {
                    pencil.set_foreground(Color::Xterm(9));
                } else {
                    pencil.set_foreground(Color::White);
                }

                pencil.draw_char(bot.get_glyph(true), bot.position.into());
            }

            pencil.set_foreground(Color::White);
            pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 40));

            let tick_len = emulator.iterations.to_string().len();
            pencil.draw_text(
                &format!(
                    "Score: {}, Ticks: {:tick_len$} of {}   (Seed: {})",
                    emulator.tank.score,
                    emulator.current_tick,
                    emulator.iterations,
                    &args.seed.unwrap()
                ),
                Vec2::xy(0, 42)
            );

            if args.debug_bot.is_some() {
                let bot: &Bot = &emulator.bot_from_id(debug_bot_id).unwrap();

                // basic info
                pencil.draw_text(
                    &format!(
                        "[{:>5} {}] ({:2},{:2}), Energy={:5}, IP={:04}, SP={:04}, Flags={}",
                        bot.name, bot.get_glyph(false), bot.position.x, bot.position.y, bot.energy, bot.instruction_pointer, bot.stack_pointer, bot.flags
                    ),
                    Vec2::xy(0, 44)
                );

                // registers
                pencil.draw_text(
                    &format!("R00={:5} R01={:5} R02={:5} R03={:5} R04={:5} R05={:5} R06={:5}",
                             bot.registers[0], bot.registers[1],
                             bot.registers[2], bot.registers[3],
                             bot.registers[4], bot.registers[5], bot.registers[6]),
                    Vec2::xy(0, 45)
                );
                pencil.draw_text(
                    &format!("R07={:5} R08={:5} R09={:5} R10={:5} R11={:5} R12={:5} R13={:5}",
                             bot.registers[7], bot.registers[8],
                             bot.registers[9], bot.registers[10],
                             bot.registers[11], bot.registers[12], bot.registers[13]),
                    Vec2::xy(0, 46)
                );

                // next instruction
                pencil.draw_text(
                    &format!("{:04}  {}", bot.instruction_pointer, Disassembler::parse(bot.get_instruction(), bot.instruction_pointer, true)),
                    Vec2::xy(0, 47)
                );

                pencil.draw_text(
                    "(u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]: <WIP>",
                    Vec2::xy(0, 48)
                );


                pencil.draw_text(
                    &format!("Toxic Sludge: {:?} of {}", emulator.tank.toxic_sludge, emulator.tank.sludge_types),
                    Vec2::xy(0, 50)
                );
            }



            pencil.draw_text(
                &format!("", ),
                Vec2::xy(0, 48)
            );
        });
    }

    if emulator.finished {
        let (active_bots, active_drones) = emulator.active_bot_count();

        println!("Bot Info: <not implemented>"); // TODO: grab info line
        println!("Score: {}", emulator.tank.score);
        println!(
            "Active bots: {}, Active drones: {}, Final tick: {}, Seed: {}",
            active_bots,
            active_drones,
            emulator.current_tick,
            emulator.rng.get_seed()
        )
    }
}
