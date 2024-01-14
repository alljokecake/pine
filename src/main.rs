use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap()]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Authenticate to deploy and manage squids 
    Auth(AuthArgs),
    /// Return user and context
    Whoami(WhoamiArgs),
    /// Return logs
    Logs(LogsArgs),
    /// Create a squid from template
    Init(InitArgs),
}

#[derive(Args, Debug)]
struct AuthArgs {
    #[arg(short, long)]
    key: Option<String>,
}

#[derive(Args, Debug)]
struct LogsArgs {
    #[arg(short, long)]
    container: Option<String>,

    #[arg(short, long)]
    follow: Option<String>,

    #[arg(short, long)]
    level: Option<String>,

    #[arg(short, long)]
    pagesize: Option<String>,

    #[arg(short, long)]
    since: Option<String>,
}

#[derive(Args, Debug)]
struct WhoamiArgs {}

#[derive(Args, Debug)]
struct InitArgs {}

fn main() {
    let cli = Opts::parse(); // use anyhow

    match &cli.command {
        Commands::Auth(options) => {
            println!("Authentication Key: {:?}", options.key);
        },
        Commands::Logs(options) => {
            println!("Container: {:?}", options.container);
        },
        Commands::Whoami(_) => {
            println!("who do you think you are talking to rn?");
        },
        _ => ()
    }
}
