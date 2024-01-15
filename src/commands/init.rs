use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {

    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn init(args: &InitArgs) {
    println!("{:?}", args.default);
}
