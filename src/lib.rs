
pub mod commands;
pub mod cli;

pub use crate::commands::custom::CustomCmd;
// pub use crate::commands::hash::HashArgs;
pub use crate::commands::std::StdCmd;
pub use crate::commands::uuid::UuidCmd;

pub use cli::*;