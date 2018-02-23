use db::DbConfig;
use std::fs::File;
use std::io::Read;

use toml;

use error::Result;
pub const CFG_DEFAULT_PATH: &'static str = "/hab/svc/rocket-diesel-demo/config.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub db: DbConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db: DbConfig::default(),
        }
    }
}

impl Config {
    pub fn from_file(filepath: &str) -> Result<Self> {
        println!("Config File Location: {:?}", filepath);
        let mut file = File::open(filepath)?;
        let mut raw = String::new();
        file.read_to_string(&mut raw)?;
        Self::from_raw(&raw)
    }

    fn from_raw(raw: &str) -> Result<Self> {
        let value = toml::from_str(&raw)?;
        Ok(value)
    }}