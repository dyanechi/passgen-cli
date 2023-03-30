use clap::{Parser, Args, Subcommand, ValueEnum};
use rand::thread_rng;

use crate::{commands::{uuid::{UuidCommand, random_uuid}, hash::HashCmd}, UuidCmd, StdCmd, CustomCmd};

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
    
        let commands = match cli.commands {
            Some(commands) => commands,
            None => CliCommands::Standard(Default::default()),
        };
    
        match commands {
            CliCommands::Standard(cmd) => cmd.run(&mut rng),
            CliCommands::Uuid(cmd) => cmd.run(),
            CliCommands::Hash(cmd) => cmd.run(),
            CliCommands::Custom(cmd) => cmd.run(),
        }
    }
}

#[derive(Args, Clone, Debug, Default)]
pub struct CliArgs {
    
}

#[derive(Subcommand, Clone, Debug)]
enum CliCommands {
    Standard(StdCmd),
    Uuid(UuidCmd),
    Hash(HashCmd),
    Custom(CustomCmd),
}
