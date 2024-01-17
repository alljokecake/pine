pub const DEFAULT_API_URL: &str = "https://saas.infra.gc.subsquid.io/api";

pub struct Config {
    pub api_url: String,
    pub credentials: String,
}

pub fn set_config(key: &String, host: &String) {}
