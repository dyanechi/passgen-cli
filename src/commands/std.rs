use super::*;
use rand::{Rng, rngs::ThreadRng};

type ModeFlags = (bool, bool, bool, bool);

const NUMERIC: &'static str = "0123456789";
const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_CHARACTERS: &'static str = "!@#$%^&*()_-+";
const ALPHANUMERIC: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Args, Clone, Debug, Default)]
pub struct StdCmd {
    #[arg(short='L', long, default_value_t = 16)]
    length: usize,

    #[arg(short, long, value_enum, default_value = None)]
    mode: Option<Mode>,

    #[arg(long)]
    pre: Option<String>,

    #[arg(long)]
    post: Option<String>,

    #[arg(short, long, default_value_t = false)]
    numeric: bool,
    #[arg(short, long, default_value_t = false)]
    upper: bool,
    #[arg(short, long, default_value_t = false)]
    lower: bool,
    #[arg(short, long, default_value_t = false)]
    special: bool,

    #[arg(short, long, default_value = None)]
    custom: Option<String>,
}
impl StdCmd {
    pub fn run(self, rng: &mut ThreadRng) {
        let args = self;
        let mode_flags = (args.numeric, args.upper, args.lower, args.special);
        let mut r_str = match args.mode {
            Some(mode) => random_string_mode(rng, &mode, args.length, args.custom),
            None => random_string_flags(rng, mode_flags, args.length, args.custom)
        };
    
        if let Some(pre) = &args.pre { r_str = pre.to_owned() + &r_str; }
        if let Some(post) = &args.post { r_str.push_str(post); }
    
        println!("{}", r_str);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, ValueEnum)]
pub enum Mode {
    Num,
    Lower,
    Upper,
    Special,
    Alpha,
    Alnum,
    All,
    Uuid,
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
        Mode::Uuid => format!("{}{}", LOWERCASE, UPPERCASE),
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