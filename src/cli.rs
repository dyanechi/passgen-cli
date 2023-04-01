use super::commands::*;

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
    command: Option<CliCommand>
}
impl Cli {
    pub fn run() {
        let cli = Cli::parse();
        let mut rng = thread_rng();
    
        let args = cli.args;
        let command = match cli.command {
            Some(command) => command,
            None => CliCommand::Standard(StdCmd::from(args.std_args)),
        };
    
        match command {
            CliCommand::Standard(cmd) => cmd.run(&mut rng),
            CliCommand::Uuid(cmd) => cmd.run(),
            CliCommand::Hash(cmd) => cmd.run(),
            CliCommand::Custom(cmd) => cmd.run(),
        }

    }
}

#[derive(Args, Clone, Debug, Default)]
pub struct CliArgs {
    #[clap(flatten)]
    std_args: StdArgs,
}

#[derive(Subcommand, Clone, Debug)]
enum CliCommand {
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
