use clap::{Parser, Subcommand};

use crate::commands::auth::AuthArgs;
use crate::commands::whoami::WhoamiArgs;

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
    /// Return user and context
    Whoami(WhoamiArgs),
}
