use clap::Args;

#[derive(Args, Debug)]
pub struct ProdArgs {
    /// name@version
    #[arg()]
    pub name_and_version: Option<String>,
}

pub fn prod(_args: &ProdArgs) {
    todo!();
}
