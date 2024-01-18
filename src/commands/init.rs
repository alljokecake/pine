use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(long_help = concat!(
            "The squid name. It must contain only alphanumeric or dash ('-')\n",
            "symbols and must not start with '-'. Squid names are globally\n",
            "unique."
            ))]
    pub name: Option<String>,

    #[arg(short, long)]
    #[arg(long_help = concat!(
            "The target location for the squid. If omitted, a new folder NAME\n", 
            "is created."
            ))]
    pub dir: Option<PathBuf>,

    #[arg(short, long, num_args = 0)]
    #[arg(help = "Clean up the target directory if exists")]
    pub remove: (),

    #[arg(short, long)]
    #[arg(long_help = concat!(
            "A template for the squid. Accepts:\n",
            "- a github repository URL containing a valid squid.yaml manifest\n",
            "  in the root folder or one of the pre-defined aliases:\n",
            "- evm  A minimal squid template for indexing EVM data.\n",
            "- abi  A template to auto-generate a squid indexing events and txs\n",
            "  from a contract ABI\n",
            "- multichain  A template for indexing data from multiple chains\n",
            "- gravatar  A sample EVM squid indexing the Gravatar smart contract\n",
            "  on Ethereum.\n",
            "- substrate  A template squid for indexing Substrate-based chains.\n",
            "- ink  A template for indexing Ink! smart contracts\n",
            "- ink-abi  A template to auto-generate a squid from an ink!\n",
            "  contract ABI\n",
            "- frontier-evm  A template for indexing Frontier EVM chains,\n",
            "  like Moonbeam and Astar.\n",
            ))]
    pub template: Option<String>,
}

pub fn init(_args: &InitArgs) {
    todo!();
}
