use crate::config::config::{set_config, DEFAULT_API_URL};
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
