use std::path::PathBuf;
use clap::Args;

#[derive(Args, Debug)]
pub struct DeployArgs {

    #[arg(short, long, default_value = "squid.yaml")]
    pub manifest: PathBuf,
}

pub fn deploy(args: &DeployArgs) {
    println!("Manifest: {:?}", args.manifest);
}
