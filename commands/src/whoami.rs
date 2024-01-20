use clap::Args;

#[derive(Args, Debug)]
pub struct WhoamiArgs {}

pub fn whoami() {
    println!("Heisenberg.");
}
