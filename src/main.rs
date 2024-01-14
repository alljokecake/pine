mod commands;
mod opts;

use clap::Parser;
use opts::{Opts, Commands};
use commands::{
    auth::auth, 
    whoami::whoami,
};


fn main() {
    let cli = Opts::parse(); // use anyhow

    match &cli.command {
        Commands::Auth(options) => {
            auth(options);
        },
        Commands::Whoami(_) => {
            whoami();
        },
    }
}
