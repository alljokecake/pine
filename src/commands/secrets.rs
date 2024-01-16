use clap::Args;

#[derive(Args, Debug)]
pub struct SecretsArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn secrets(args: &SecretsArgs) {
    println!("{:?}", args.default);
}
