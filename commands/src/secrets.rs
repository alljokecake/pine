use clap::Args;

// TODO: secrets:ls secrets:rm 
#[derive(Args, Debug)]
pub struct SecretsArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn secrets(_args: &SecretsArgs) {
    todo!();
}
