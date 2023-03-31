use super::*;

#[derive(Args, Clone, Debug)]
pub struct HashArgs {
    #[arg(short, long)]
    func: Option<HashFunction>,

    #[clap(flatten)]
    pub shared: SharedArgs,
}

#[derive(Args, Clone, Debug)]
pub struct HashCmd {

    #[clap(flatten)]
    args: HashArgs,
}
impl HashCmd {
    pub fn run(self) {
        let args = self.args;

        eprintln!("Warn: Not implemented yet!");
    }
}


#[derive(ValueEnum, Clone, Debug)]
enum HashFunction {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Sha3_512_224,
    Sha3_512_256,
    Sha3_512_512,
}