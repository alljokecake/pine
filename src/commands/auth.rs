use crate::config::config::{DEFAULT_API_URL, set_config};

use clap::Args;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[arg(short, long, required = true)]
    pub key: Option<String>,
    #[arg(long, hide = true, default_value = DEFAULT_API_URL)]
    pub host: Option<String>,
}

pub fn auth(args: &AuthArgs) {
    match (&args.key, &args.host) {
        (Some(key), Some(host)) => {
            set_config(key, host);
        },

        _ => panic!(),
    }
}
