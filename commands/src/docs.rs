use clap::Args;

#[derive(Args, Debug)]
pub struct DocsArgs {}

pub fn docs() {
    let url = "https://docs.subsquid.io/squid-cli/";

    match open::that(url) {
        Ok(_) => println!("Works"),
        Err(_) => eprintln!("You suck!"),
    }
}
