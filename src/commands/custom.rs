use super::*;

#[derive(Args, Clone, Debug)]
pub struct CustomArgs {

}

#[derive(Args, Clone, Debug)]
pub struct CustomCmd {
    #[clap(flatten)]
    args: CustomArgs,
}
impl CustomCmd {
    pub fn run(self) {
        let args = self.args;
    }
}
