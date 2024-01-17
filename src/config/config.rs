use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const DEFAULT_API_URL: &str = "https://saas.infra.gc.subsquid.io/api";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    api_url: String,
    credentials: String,
}

impl Config {
    pub fn default(api_url: &str) -> Self {
        Self {
            api_url: api_url.to_string(),
            credentials: "empty".to_string(),
        }
    }

    pub fn write_to_file(&self, path: &Path) -> io::Result<()> {
        let dir = path.parent().ok_or_else(|| io::ErrorKind::InvalidInput)?;

        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;

        Ok(())
    }

    pub fn read_from_file(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(serde_json::from_str(&content)?)
    }
}

pub fn get_config_file_path() -> PathBuf {
    let config_dir = env::var("SUBSQUID_CLI_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home_dir = dirs::home_dir().expect("Unable to determine home directory");
            home_dir.join(".hydra-cli").join("config.json")
        });

    config_dir
}

pub fn get_config() -> Config {
    match Config::read_from_file(&get_config_file_path()) {
        Ok(config) => config,
        Err(_) => Config::default(DEFAULT_API_URL),
    }
}

pub fn set_config(creds: &String, host: &String) -> Config {
    let mut config = get_config();
    config.api_url = host.to_string();
    config.credentials = creds.to_string();

    if let Err(err) = config.write_to_file(&get_config_file_path()) {
        eprintln!("Error writing config file: {}", err);
    }

    config
}
