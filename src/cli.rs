use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Arguments {
    /// Single-step debug the organism specified by X (a letter)
    #[arg(short = 'g', long = None)]
    pub debug_bot: Option<char>,

    /// Specify # of iterations
    #[arg(short = 'i', long = None, default_value = "1000000")]
    pub iterations: i32,

    /// Log organism program trace to specified file
    #[arg(short = 'l', long = None)]
    pub log_path: Option<PathBuf>,

    /// Specify the player's organism source file
    #[arg(short = 'p', long = None)]
    pub bot_path: PathBuf,

    /// Run in quiet mode (no display)
    #[arg(short = 'q', long = None, default_value_t = false)]
    pub quiet_mode: bool,

    /// Specify the randomization seed
    #[arg(short = 's', long = None)]
    pub seed: Option<i32>,

    /// Show the disassembly and bytecode for this organism
    #[arg(short = 'z', long = None, default_value_t = false)]
    pub show_disassembly: bool,
}