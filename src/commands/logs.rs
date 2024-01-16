use clap::Args;

#[derive(Args, Debug)]
pub struct LogsArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn logs(args: &LogsArgs) {
    println!("{:?}", args.default);
}
