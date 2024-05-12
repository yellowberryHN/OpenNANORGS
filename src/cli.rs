use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Arguments {
    /// Specify the player's organism source file
    #[arg(value_name="BOT", value_hint = ValueHint::FilePath)]
    pub bot_path: PathBuf,

    /// This allows you to use the old argument format
    #[arg(short = 'p', hide = true)]
    legacy_player_flag: bool,

    /// Run in quiet mode (no display)
    #[arg(short = 'q', long = None, default_value_t = false)]
    pub quiet_mode: bool,

    /// Single-step debug the organism specified by X (a letter)
    #[arg(short = 'g', long = None, value_name="CHAR")]
    pub debug_bot: Option<char>,

    /// Specify # of iterations
    #[arg(short = 'i', long = None, default_value = "1000000", value_name="NUM")]
    pub iterations: i32,

    /// Log organism program trace to specified file
    #[arg(short = 'l', long = None, value_name="PATH")]
    pub log_path: Option<PathBuf>,

    /// Specify the randomization seed
    #[arg(short = 's', long = None)]
    pub seed: Option<i32>,

    /// Show the disassembly and bytecode for this organism
    #[arg(short = 'z', long = None, default_value_t = false)]
    pub show_disassembly: bool,

    /// Dump bytecode into firmware file
    #[arg(short = 'f', long = None, default_value_t = false)]
    pub dump_bytecode: bool,

    /// Dump bytecode into firmware file as text
    #[arg(long = "dump-bytecode-text", default_value_t = false, hide = true)]
    pub dump_bytecode_text: bool,

    /// Display additional logs for troubleshooting (internal)
    #[arg(short = 'v', hide = true)]
    pub verbose: bool,
}

