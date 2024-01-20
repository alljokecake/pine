use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg()]
    pub path: Option<PathBuf>,

    /// Do not run specified services
    #[arg(short, long)]
    pub exclude: Option<String>,

    /// Relative path to an additional env file in squid source
    #[arg(short, long = "envFile", default_value = ".env")]
    pub file: Option<PathBuf>,

    /// Run only specified services
    #[arg(short, long)]
    pub include: Option<String>,

    /// Relative path to a squid manifest file in squid source
    #[arg(short, long, default_value = "squid.yaml")]
    pub manifest: Option<String>,

    /// Attempts to restart failed or stopped services
    #[arg(short, long, default_value = "5")]
    pub retries: Option<u8>,
}

pub fn run(_args: &RunArgs) {
    todo!();
}
