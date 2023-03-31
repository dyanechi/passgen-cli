
trait RunCommand {
    fn run(self, shared_args: SharedArgs);
}

mod commands;
pub mod cli;
pub mod prelude;

pub use cli::*;
// pub use prelude::*;