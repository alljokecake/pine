use clap::{Parser, Subcommand};

use crate::commands::{
    auth::AuthArgs,
    deploy::DeployArgs,
    explorer::ExplorerArgs,
    init::InitArgs,
    logs::LogsArgs,
    ls::LsArgs,
    prod::ProdArgs,
    restart::RestartArgs,
    rm::RmArgs,
    run::RunArgs,
    secrets::SecretsArgs,
    whoami::WhoamiArgs,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap()]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Authenticate to deploy and manage squids
    Auth(AuthArgs),

    /// Deploy a new or update an existing squid version
    Deploy(DeployArgs),

    /// Visual explorer of deployed squids
    Explorer(ExplorerArgs),

    /// Create a squid from a template
    Init(InitArgs),

    /// Fetch squid logs
    Logs(LogsArgs),

    /// List squids and squid versions
    Ls(LsArgs),

    /// Assign squid version to the production endpoint
    Prod(ProdArgs),

    /// Restart a squid version
    Restart(RestartArgs),

    /// Remove a squid or squid version
    Rm(RmArgs),

    /// Run a squid locally according to the deployment manifest
    Run(RunArgs),

    /// Manage Account secrets
    Secrets(SecretsArgs),

    /// Return user and context
    Whoami(WhoamiArgs),
}
