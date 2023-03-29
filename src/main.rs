use clap::Parser;
use passgen_cli::{Mode, random_string_mode, random_string_flags};
use rand::thread_rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short='L', long, default_value_t = 16)]
    length: usize,

    #[arg(short, long, value_enum)]
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

fn main() {
    let args = Cli::parse();
    let mut rng = thread_rng();

    let mode_flags = (args.numeric, args.upper, args.lower, args.special);
    let mut r_str = match args.mode {
        Some(mode) => random_string_mode(&mut rng, &mode, args.length, args.custom),
        None => random_string_flags(&mut rng, mode_flags, args.length, args.custom)
    };

    if let Some(pre) = &args.pre { r_str = pre.to_owned() + &r_str; }
    if let Some(post) = &args.post { r_str.push_str(post); }

    println!("{}", r_str);
}
