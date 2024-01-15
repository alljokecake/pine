use clap::Args;

#[derive(Args, Debug)]
pub struct ProdArgs {

    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn prod(args: &ProdArgs) {
    println!("{:?}", args.default);
}
