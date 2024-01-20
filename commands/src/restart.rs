use clap::Args;

#[derive(Args, Debug)]
pub struct RestartArgs {
    /// Don't attach and stream squid logs after the deploy
    #[arg(long = "no-stream-logs", num_args = 0)]
    pub no_stream_logs: (),
}

pub fn restart(_args: &RestartArgs) {
    todo!();
}
