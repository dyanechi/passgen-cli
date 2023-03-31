use crate::RunCommand;

use super::prelude::*;

use clap::{Parser, Args, Subcommand};
use rand::thread_rng;


#[derive(Parser, Debug)]
#[command(
    author = "Dawid Janeczko",
    version = "0.1.0",
    about = "Rust Cli for generating random passwords and uuid",
    long_about = None
)]
#[clap(args_conflicts_with_subcommands = true)]
#[clap(subcommand_negates_reqs = true)]
pub struct Cli {
    #[clap(flatten)]
    args: CliArgs,

    #[command(subcommand)]
    commands: Option<CliCommands>
}
impl Cli {
    pub fn run() {
        let cli = Cli::parse();
        let mut rng = thread_rng();
    
        let args = cli.args;
        let shared = args.shared_args;
        let commands = match cli.commands {
            Some(commands) => commands,
            None => CliCommands::Standard(StdCmd::from(args.std_args)),
        };
    
        match commands {
            CliCommands::Standard(cmd) => cmd.run(shared, &mut rng),
            CliCommands::Uuid(cmd) => cmd.run(shared),
            CliCommands::Hash(cmd) => cmd.run(shared),
            CliCommands::Custom(cmd) => cmd.run(shared),
        }

    }
}

#[derive(Args, Clone, Debug, Default)]
pub struct CliArgs {
    #[clap(flatten)]
    std_args: Option<StdArgs>,

    #[clap(flatten)]
    shared_args: SharedArgs,
}

#[derive(Subcommand, Clone, Debug)]
enum CliCommands {
    Standard(StdCmd),
    Uuid(UuidCmd),
    Hash(HashCmd),
    Custom(CustomCmd),
}

#[derive(Args, Clone, Debug, Default)]
pub struct SharedArgs {
    #[arg(short='Q', long, default_value_t = 1)]
    pub quantity: usize,
}
