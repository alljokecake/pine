use clap::Args;

#[derive(Args, Debug)]
pub struct RestartArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn restart(_args: &RestartArgs) {
    todo!();
}
