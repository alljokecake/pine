use clap::Args;

#[derive(Args, Debug)]
pub struct RmArgs {
    /// <name> or <name@version>
    #[arg()]
    pub name_and_version: Option<String>,

    /// Does not prompt before removing a squid or its version
    #[arg(short, long, num_args = 0)]
    pub force: (),
}

pub fn rm(_args: &RmArgs) {
    todo!();
}
