use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {

    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn run(args: &RunArgs) {
    println!("{:?}", args.default);
}
