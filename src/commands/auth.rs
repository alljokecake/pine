use clap::Args;

#[derive(Args, Debug)]
pub struct AuthArgs {
    #[arg(short, long)]
    pub key: Option<String>,
}

pub fn auth(key: &AuthArgs) {
    println!("Authentication Key: {:?}", key.key);
}
