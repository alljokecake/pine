use clap::Args;

#[derive(Args, Debug)]
pub struct LsArgs {
    /// Squid name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Organization
    #[arg(short, long)]
    pub org: Option<String>,

    /// Truncate data in columns
    #[arg(short, long, action)]
    pub truncate: bool,
}

pub fn ls(_args: &LsArgs) {
    todo!();
}
