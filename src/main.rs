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

    if args.quiet_mode {
        let now = Instant::now();
        while emulator.current_tick < emulator.iterations {
            emulator.tick()
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

            let mut skip_draw: bool = false;

            for key in app_state.keyboard().get_keys_down() {
                match key {
                    Key::Space => {
                        skip_draw = true;
                    }
                    _ => ()
                }
            }

            if emulator.current_tick < emulator.iterations {
                emulator.tick();
            } else {
                app_state.stop()
            }

            if !skip_draw {
                fps_counter.update();

                let mut pencil = Pencil::new(window.canvas_mut());

                for element in &emulator.tank.elements {
                    let element = element.as_ref();
                    match element {
                        Some(element) => {
                            match element.item_type {
                                ItemType::Sludge => pencil.set_foreground(Color::Grey),
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
                    None => 0,
                };

                for bot in &emulator.bots {
                    if debug_bot_id > 0 && bot.id == debug_bot_id {
                        pencil.set_foreground(Color::Xterm(172));
                    } else if bot.id > 50 {
                        pencil.set_foreground(Color::Xterm(1));
                    } else {
                        pencil.set_foreground(Color::White);
                    }

                    pencil.draw_char(bot.get_glyph(), bot.position.into());
                }

                pencil.set_foreground(Color::White);
                pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 40));
                pencil.draw_text(&format!("Seed: {}", &args.seed.unwrap()), Vec2::xy(0, 41));
                pencil.draw_text(
                    &format!(
                        "Bot[0] IP: {}, SP: {}, Energy: {}, Flags: {}",
                        emulator.bots[0].instruction_pointer, emulator.bots[0].stack_pointer, emulator.bots[0].energy, emulator.bots[0].flags
                    ),
                    Vec2::xy(0, 42),
                );
                pencil.draw_text(
                    &format!("Bot[0] Registers: {:?}", emulator.bots[0].registers),
                    Vec2::xy(0, 43),
                );
                pencil.draw_text(
                    &format!("Toxic Sludge: {:?} of {}", emulator.tank.toxic_sludge, emulator.tank.sludge_types),
                    Vec2::xy(0, 44)
                );
                pencil.draw_text(
                    &format!("Ticks: {} of {}", emulator.current_tick, emulator.iterations),
                    Vec2::xy(0, 45)
                );
            }
        });
    }

    // TODO: change this condition to be after all ticks processed, regardless of amount
    if emulator.current_tick == emulator.iterations {
        let mut live_bots = 0;
        let mut live_drones = 0;

        for bot in &emulator.bots {
            match bot.id {
                1..=50 => {
                    if !bot.sleeping {
                        live_bots += 1
                    }
                }
                _ => {
                    if !bot.sleeping {
                        live_drones += 1
                    }
                }
            }
        }

        println!("Bot Info: <not implemented>"); // TODO: grab info line
        println!("Final score: {}", emulator.tank.score);
        println!(
            "Live bots: {}, Live drones: {}, Seed: {}",
            live_bots,
            live_drones,
            emulator.rng.get_seed()
        )
    }
}
