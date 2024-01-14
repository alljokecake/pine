use clap::Args;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[arg(short, long)]
    pub key: Option<String>,
}
