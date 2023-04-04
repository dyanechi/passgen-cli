use ::std::collections::HashMap;

use regex::Regex;

use crate::{commands::std::{random_string_flags, DistSetFlags}, util::rand_number};

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

        let parsed = rand_from_pattern(&pat.trim());

        println!("{}", parsed);
    }
}

pub fn rand_from_pattern(pat: &str) -> String {
    const TOKEN_PATTERN: &'static str = r"\$\{(((N)?(U)?(L)?(S)?(H)?(O)?(B)?(A)?):(([0-9]*)(-([0-9]*))?))\}";
    let mut rng = rand::thread_rng();
    let mut parsed_result = pat.to_string();

    let re = Regex::new(TOKEN_PATTERN).unwrap();
    let matches = re.captures_iter(pat);

    for mat in matches {
        // Extract flags and length of random characters to fill
        // println!("{:#?}", mat);
        let repl_pat = mat[0].to_owned();
        if repl_pat.is_empty() { continue; }
        
        let user_flags = mat[2].to_owned();
        let min: usize = mat.get(mat.len()-3).expect("should have minimum").as_str().parse().unwrap_or_default();
        if min <= 0 {
            panic!("ERROR: No quantity provided in pattern '{repl_pat}'. Use format 'FLAGS:UINT' instead.\nExample: 'NU:12' - generates 12 characters NUM+UPPER (W0M5DZ44YSFV)");
        }

        let mut quantity = min;
        if let Some(max) = mat.get(mat.len()-1) {
            let max = max.as_str().parse().unwrap_or_default();
            if max <= min { panic!("ERROR: max should be higher than min") }
            quantity = rand_number(&mut rng, min, max);
        }

        // Fill in the flags
        let flags = DistSetFlags::from(&user_flags);
        let value = random_string_flags(&mut rng, flags, quantity, None);

        parsed_result = parsed_result.replacen(&repl_pat, &value, 1);
    }

    parsed_result
}
