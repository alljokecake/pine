use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct DeployArgs {
    /// Squid source
    #[arg()]
    pub source: Option<String>,

    /// Relative path to a squid manifest file
    #[arg(short, long, default_value = "squid.yaml")]
    pub manifest: Option<PathBuf>,

    /// Organization
    #[arg(short, long)]
    pub org: Option<String>,

    /// Do a hard reset before deploying
    #[arg(short, long = "hard-reset", num_args = 0)]
    pub reset: (),

    /// Don't require a confirmation if the version already exists
    #[arg(short, long, num_args = 0)]
    pub update: (),

    /// Don't attach and stream squid logs after the deploy
    #[arg(long = "no-stream-logs", num_args = 0)]
    pub no_stream_logs: (),
}

pub fn deploy(_args: &DeployArgs) {
    todo!();
}
