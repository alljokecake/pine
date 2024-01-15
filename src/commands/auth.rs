use clap::Args;

#[derive(Args, Debug)]
pub struct AuthArgs {

    #[arg(short, long)]
    pub key: Option<String>,
}

pub fn auth(args: &AuthArgs) {
    println!("Authentication Key: {:?}", args.key);
}
