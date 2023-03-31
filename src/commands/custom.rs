use super::*;

#[derive(Args, Clone, Debug)]
pub struct CustomArgs {
    #[clap(flatten)]
    pub shared: SharedArgs,
}

#[derive(Args, Clone, Debug)]
pub struct CustomCmd {
    #[clap(flatten)]
    args: CustomArgs,
}
impl CustomCmd {
    pub fn run(self) {
        let args = self.args;

        eprintln!("Warn: Not implemented yet!");
    }
}
