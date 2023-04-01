use clap::{Args, ValueEnum};
use crate::SharedArgs;

mod custom;
mod hash;
mod std;
mod uuid;

pub use {
    self::std::{StdCmd, StdArgs},
    self::uuid::{UuidCmd, UuidArgs},
    self::hash::{HashCmd, HashArgs},
    self::custom::{CustomCmd, CustomArgs},
};