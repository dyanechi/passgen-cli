use super::*;

extern crate uuid;
use clap::Subcommand;
use uuid::Uuid;

#[derive(Args, Clone, Debug)]
pub struct UuidCmd {
    #[command(subcommand)]
    pub version: Option<UuidCommand>,

    #[clap(flatten)]
    pub args: UuidArgs,
}
impl UuidCmd {
    pub fn run(self) {
        let args = self.args;
        let version = self.version.unwrap_or_default();
        (0..args.shared.quantity).for_each(|_| 
            println!("{}", random_uuid(&version))
        );
    }
}

#[derive(Args, Clone, Debug)]
pub struct UuidArgs {
    #[clap(flatten)]
    pub shared: SharedArgs,
}

#[derive(Debug, Default, Clone, Subcommand)]
pub enum UuidCommand {
    // V1,
    // V3,
    #[default]
    V4,
    V5(V5Args),
    // V6,
    // V7,
    // V8,
}

#[derive(Debug, Default, Clone, Args)]
pub struct V5Args {
    #[arg(short='d', long)]
    domain: String,
}

pub fn random_uuid(uuid_ver: &UuidCommand) -> String {
    // match uuid_ver {
    //     // UuidVersion::V1 => fallback_uuid(),
    //     // UuidVersion::V3 => fallback_uuid(),
    //     UuidVersion::V4 => Uuid::new_v4().to_string(),
    //     UuidVersion::V5 => Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"rust-lang.org").to_string(),
    //     // UuidVersion::V6 => fallback_uuid(),
    //     // UuidVersion::V7 => fallback_uuid(),
    //     // UuidVersion::V8 => fallback_uuid(),
    //     _ => fallback_uuid(),
    // }
    match uuid_ver {
        UuidCommand::V4 => Uuid::new_v4().to_string(),
        UuidCommand::V5(args) => Uuid::new_v5(&Uuid::NAMESPACE_DNS, args.domain.as_bytes()).to_string(),
        // UuidCommand::V6 => ,
        // UuidCommand::V7 => fallback_uuid(),
        // UuidCommand::V8 => fallback_uuid(),
    }
}

// fn fallback_uuid() -> String {
//     eprintln!("WARN: This uuid version is not supported. Falling back to uuid_v4");
//     Uuid::new_v4().to_string()
// }