use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The squid name
    #[arg()]
    pub name: Option<String>,
    /// Target location for the squid.
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
    /// Clean up the target directory if exists
    #[arg(short, long, num_args = 0)]
    pub remove: (),
    /// A template for the squid
    #[arg(short, long)]
    pub template: Option<String>,
}

pub fn init(args: &InitArgs) {
    todo!();
}
