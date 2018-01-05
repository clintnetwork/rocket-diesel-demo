use db::DbConfig;
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::DeserializeOwned;
use toml;
use error::{Error, Result};

#[derive(Debug, Deserialize)]
#[serde(default)]
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

impl FromStr for Config {
    type Err = Error;

    fn from_str(toml: &str) -> Result<Self> {
        let config: Config = toml::from_str(toml).unwrap();
        Ok(config)
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

pub trait ConfigFile: DeserializeOwned + Sized {
    type Error: From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> Result<Self> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => {
                return Err(Self::Error::from(
                    Error::ConfigFileIO(filepath.as_ref().to_path_buf(), e),
                ))
            }
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => {
                return Err(Self::Error::from(
                    Error::ConfigFileIO(filepath.as_ref().to_path_buf(), e),
                ))
            }
        }
        Self::from_raw(&raw)
    }

    fn from_raw(raw: &str) -> Result<Self> {
        let value = toml::from_str(&raw).map_err(|e| Error::ConfigFileSyntax(e))?;
        Ok(value)
    }
}
