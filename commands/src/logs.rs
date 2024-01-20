use clap::Args;

#[derive(Args, Debug)]
pub struct LogsArgs {
    /// name@version
    #[arg()]
    pub name: Option<String>,

    /// options: [processor|api|db-migrate|db]
    #[arg(short, long, value_name = "options")]
    pub container: Option<String>,

    /// options: [error|debug|info|warning]
    #[arg(short, long, value_name = "options")]
    pub level: Option<String>,

    /// Logs page size
    #[arg(short, long, default_value = "100")]
    pub page_size: Option<String>,

    /// Filter by date/interval
    #[arg(long, default_value = "1d")]
    pub since: Option<String>,
}

pub fn logs(_args: &LogsArgs) {
    todo!();
}
