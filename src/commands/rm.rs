use clap::Args;

#[derive(Args, Debug)]
pub struct RmArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn rm(args: &RmArgs) {
    println!("{:?}", args.default);
}
