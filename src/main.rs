mod config;
mod commands;
mod opts;

use clap::Parser;
use commands::{
    auth::auth,
    deploy::deploy,
    explorer::explorer,
    init::init,
    logs::logs,
    ls::ls,
    prod::prod,
    restart::restart,
    rm::rm,
    run::run,
    secrets::secrets,
    whoami::whoami,
};
use opts::{Commands, Opts};

fn main() {
    let opts = Opts::parse();

    match &opts.command {
        Commands::Auth(args) => auth(args),
        Commands::Deploy(args) => deploy(args),
        Commands::Explorer(args) => explorer(args),
        Commands::Init(args) => init(args),
        Commands::Logs(args) => logs(args),
        Commands::Ls(args) => ls(args),
        Commands::Prod(args) => prod(args),
        Commands::Restart(args) => restart(args),
        Commands::Rm(args) => rm(args),
        Commands::Run(args) => run(args),
        Commands::Secrets(args) => secrets(args),
        Commands::Whoami(_) => whoami(),
    }
}
