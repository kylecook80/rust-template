use crate::error;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {}

const CONFIG_NAME: &str = "{{project-name}}.toml";
static FILES: OnceLock<Vec<PathBuf>> = OnceLock::new();

impl Config {
    /// Reads config from /etc/mercury.toml and parses it to a struct.
    // In theory we can have an option arg here to accept a config passed via --flag at runtime
    pub fn new(file: Option<PathBuf>) -> Result<Config, error::Error> {
        tracing::debug!("Config being parsed");

        let v = vec![
            Path::new("./").join(CONFIG_NAME),
            Path::new("/etc").join(CONFIG_NAME),
        ];

        let _ = FILES.set(v);

        let mut config_path;
        if let Some(p) = file {
            config_path = p;
        } else {
            config_path = PathBuf::from(FILES.get().unwrap().last().unwrap());

            for file in FILES.get().unwrap() {
                if Path::new(file).exists() {
                    let pathbuf = PathBuf::from(file);
                    config_path = pathbuf;
                }
            }
        }

        if !config_path.exists() {
            return Err(error::Error::NoConfig);
        }

        let mut config_file = File::open(config_path)?;
        let mut buf = String::new();
        let _bytes = config_file.read_to_string(&mut buf);
        let res: Result<Config, toml::de::Error> = toml::from_str(&buf);

        match res {
            Ok(t) => Ok(t),
            Err(e) => Err(error::Error::ConfigParse(e)),
        }
    }
}
