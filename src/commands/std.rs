use super::*;
use rand::{Rng, rngs::ThreadRng};

type ModeFlags = (bool, bool, bool, bool);

const BINARY: &'static str = "01";
const OCTAL: &'static str = "01234567";
const NUMERIC: &'static str = "0123456789";
const HEX: &'static str = "0123456789abcdef";
const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_CHARACTERS: &'static str = "!@#$%^&*()_-+";
const ALPHANUMERIC: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const DEFAULT_LEN: usize = 16;

#[derive(Args, Clone, Debug, Default)]
pub struct DistSetFlags {
    #[arg(short, long, default_value_t = false)]
    pub lower: bool,
    #[arg(short, long, default_value_t = false)]
    pub upper: bool,
    #[arg(short, long, default_value_t = false)]
    pub special: bool,
    #[arg(short, long, default_value_t = false)]
    pub numeric: bool,
    #[arg(short, long, default_value_t = false)]
    pub hex: bool,
    #[arg(short, long, default_value_t = false)]
    pub octal: bool,
    #[arg(short, long, default_value_t = false)]
    pub binary: bool,
    #[arg(short, long, default_value = "")]
    pub custom: String,
}
impl DistSetFlags {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_lower(mut self) -> Self { self.lower = true; self }
    pub fn with_upper(mut self) -> Self { self.upper = true; self }
    pub fn with_special(mut self) -> Self { self.special = true; self }
    pub fn with_numeric(mut self) -> Self { self.numeric = true; self }
    pub fn with_hex(mut self) -> Self { self.hex = true; self }
    pub fn with_octal(mut self) -> Self { self.octal = true; self }
    pub fn with_binary(mut self) -> Self { self.binary = true; self }
    pub fn with_custom(mut self, custom: Option<String>) -> Self { self.custom = custom.unwrap_or_default(); self }

    pub fn charset(&self) -> String {
        let mut charset = String::new();

        if self.lower { charset.push_str(LOWERCASE); }
        if self.upper { charset.push_str(UPPERCASE); }
        if self.special { charset.push_str(SPECIAL_CHARACTERS); }
        if self.numeric { charset.push_str(NUMERIC); }
        else if self.hex { charset.push_str(HEX); }
        else if self.octal { charset.push_str(OCTAL); }
        else if self.binary { charset.push_str(BINARY); }

        if charset.is_empty() { ALPHANUMERIC.to_string() } else { charset }
    }
}
impl From<&String> for DistSetFlags {
    fn from(value: &String) -> Self {
        let mut flags = Self::new();
        for v in value.to_owned().chars() {
            match v {
                'l' | 'L' => flags.lower = true,
                'u' | 'U' => flags.upper = true,
                's' | 'S' => flags.special = true,
                'n' | 'N' => flags.numeric = true,
                'h' | 'H' => flags.hex = true,
                'o' | 'O' => flags.octal = true,
                'b' | 'B' => flags.binary = true,
                'a' | 'A' => {
                    flags.lower = true;
                    flags.upper = true;
                    flags.numeric = true;
                }
                _ => { eprintln!("WARN: pattern '{v}' is not recognized"); },
            }
        }
        flags
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, ValueEnum)]
pub enum Mode {
    Num,
    Lower,
    Upper,
    Special,
    Alpha,
    Alnum,
    All,
}

#[derive(Args, Clone, Debug, Default)]
pub struct StdArgs {
    #[clap(flatten)]
    pub shared: SharedArgs,

    #[arg(short='L', long, default_value_t = 16)]
    length: usize,

    #[arg(short, long, value_enum, default_value = None)]
    mode: Option<Mode>,

    #[arg(long)]
    pre: Option<String>,

    #[arg(long)]
    post: Option<String>,

    #[clap(flatten)]
    flags: DistSetFlags,

    #[arg(short, long, default_value = None)]
    custom: Option<String>,
}

#[derive(Args, Clone, Debug, Default)]
pub struct StdCmd {
    #[clap(flatten)]
    args: StdArgs,
}
impl From<Option<StdArgs>> for StdCmd {
    fn from(args: Option<StdArgs>) -> Self {
        let mut instance = Self::default();
        instance.args = args.unwrap_or_default();
        instance
    }
}
impl From<StdArgs> for StdCmd {
    fn from(args: StdArgs) -> Self {
        let mut instance = Self::default();
        instance.args = args;
        instance
    }
}
impl StdCmd {
    pub fn run(self, rng: &mut ThreadRng) {
        let args = self.args;
        for _ in 0..args.shared.quantity {
            let mut r_str = match args.mode {
                Some(mode) => random_string_mode(rng, &mode, args.length, args.custom.clone()),
                None => random_string_flags(rng, args.flags.clone(), args.length, args.custom.clone())
            };
        
            if let Some(pre) = &args.pre { r_str = pre.to_owned() + &r_str; }
            if let Some(post) = &args.post { r_str.push_str(post); }
        
            println!("{}", r_str);
        }
    }
}


fn random_string(rng: &mut ThreadRng, len: usize, charset: &str) -> String {
    let len = if len == 0 { DEFAULT_LEN } else { len };
    (0..len).map(
        |_| (charset.as_bytes()[rng.gen_range(0..charset.len())] as char).to_string()
    ).collect::<Vec<_>>().concat().to_owned()
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

pub fn random_string_flags(rng: &mut ThreadRng, dist_flags: DistSetFlags, len: usize, custom_dist: Option<String>) -> String {
    let charset = dist_flags.with_custom(custom_dist).charset();
    random_string(rng, len, &charset)
}
