use clap::Args;

#[derive(Args, Debug)]
pub struct ExplorerArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn explorer(args: &ExplorerArgs) {
    todo!();
}
