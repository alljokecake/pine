use clap::Args;

#[derive(Args, Debug)]
pub struct LsArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn ls(_args: &LsArgs) {
    todo!();
}
