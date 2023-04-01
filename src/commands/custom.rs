use ::std::collections::HashMap;

use regex::Regex;

use crate::commands::std::{random_string_flags, DistSetFlags};

use super::*;

#[derive(Args, Clone, Debug)]
pub struct CustomArgs {
    #[clap(flatten)]
    pub shared: SharedArgs,

    #[arg(short, long)]
    pattern: String,
}

#[derive(Args, Clone, Debug)]
pub struct CustomCmd {
    #[clap(flatten)]
    args: CustomArgs,
}
impl CustomCmd {
    pub fn run(self) {
        let args = self.args;
        let pat = args.pattern.clone();
        if pat.is_empty() { panic!("Pattern must not be empty") }

        let parsed = parse(&pat.trim());

        println!("{}", parsed);
    }
}

fn parse(pat: &str) -> String {
    const TOKEN_PATTERN: &'static str = r"(((N)?(U)?(L)?(S)?(H)?(O)?(B)?):([0-9]*))";
    let mut rng = rand::thread_rng();
    let mut parsed_result = pat.to_string();

    let re = Regex::new(TOKEN_PATTERN).unwrap();
    let matches = re.captures_iter(pat);

    let mut replacers: HashMap<String, String> = HashMap::new();
    for mat in matches {
        // Extract flags and length of random characters to fill
        // println!("{:#?}", mat);
        let repl_pat = mat[1].to_owned();
        if repl_pat.is_empty() { continue; }
        
        let user_flags = mat[2].to_owned();
        let quantity: usize = mat[mat.len()-1].to_owned().parse().unwrap_or_default();
        if quantity == 0 {
            panic!("ERROR: No quantity provided in pattern '{repl_pat}'. Use format 'FLAGS:UINT' instead.\nExample: 'NU:12' - generates 12 characters NUM+UPPER (W0M5DZ44YSFV)");
        }

        // Fill in the flags
        let flags = DistSetFlags::from(&user_flags);
        replacers.insert(repl_pat, random_string_flags(&mut rng, flags, quantity, None));
    }

    if replacers.len() == 0 {
        eprintln!("WARN: Pattern didn't contain anything to generate.");
    }

    for (k, v) in replacers.iter() {
        parsed_result = parsed_result.replace(k, v);
    }

    parsed_result
}
