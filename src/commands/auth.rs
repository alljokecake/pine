use clap::Args;
use crate::config::config::DEFAULT_API_URL;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[arg(short, long, required = true)]
    pub key: Option<String>,
    #[arg(long, hide = true, default_value = DEFAULT_API_URL)]
    pub host: Option<String>,
}

pub fn auth(args: &AuthArgs) {
    println!("Authentication Key: {:?}\nApi Url: {:?}", args.key, args.host);
}
