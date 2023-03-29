use clap::ValueEnum;
use rand::{Rng, rngs::ThreadRng, distributions::{Alphanumeric, DistString}};

type ModeFlags = (bool, bool, bool, bool);

const NUMERIC: &'static str = "0123456789";
const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_CHARACTERS: &'static str = "!@#$%^&*()_-+";
const ALPHANUMERIC: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueEnum)]
pub enum Mode {
    Num,
    Lower,
    Upper,
    Special,
    Alpha,
    Alnum,
    All,
}

pub fn random_string_mode(rng: &mut ThreadRng, mode: &Mode, len: usize, custom_dist: Option<String>) -> String {
    let charset = custom_dist.unwrap_or_else(|| match *mode {
        Mode::Num => NUMERIC.to_owned(),
        Mode::Lower => LOWERCASE.to_owned(),
        Mode::Upper => UPPERCASE.to_owned(),
        Mode::Special => SPECIAL_CHARACTERS.to_owned(),
        Mode::Alnum => ALPHANUMERIC.to_owned(),
        Mode::Alpha => format!("{}{}", LOWERCASE, UPPERCASE),
        Mode::All => format!("{}{}", ALPHANUMERIC, SPECIAL_CHARACTERS),
    });

    random_string(rng, len, &charset)
}

pub fn random_string_flags(rng: &mut ThreadRng, mode_flags: ModeFlags, len: usize, custom_dist: Option<String>) -> String {
    let charset = custom_dist.unwrap_or_else(|| {
        let mut set = String::new();
        if mode_flags.0 { set.push_str(NUMERIC) }
        if mode_flags.1 { set.push_str(UPPERCASE) }
        if mode_flags.2 { set.push_str(LOWERCASE) }
        if mode_flags.3 { set.push_str(SPECIAL_CHARACTERS) }

        if set.is_empty() { ALPHANUMERIC.to_string() } else { set }
    });

    random_string(rng, len, &charset)
}

fn random_string(rng: &mut ThreadRng, len: usize, charset: &str) -> String {
    (0..len).map(
        |_| (charset.as_bytes()[rng.gen_range(0..charset.len())] as char).to_string()
    ).collect::<Vec<_>>().concat().to_owned()
}