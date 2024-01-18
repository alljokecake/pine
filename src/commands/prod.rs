use clap::Args;

#[derive(Args, Debug)]
pub struct ProdArgs {
    #[arg(short, long)]
    pub default: Option<String>,
}

pub fn prod(_args: &ProdArgs) {
    todo!();
}
