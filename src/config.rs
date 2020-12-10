use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;

const ENV_VAR: &str = "CONFIG_PATH";
const DEFAULT_LOCATION: &str = "./config.yml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub client_id: i64,
    pub client_secret: String,
    pub domain: String,
    pub github_key: String,
}

impl Config {
    pub fn new() -> Config {
        let location = env::var(ENV_VAR).unwrap_or_else(|_|DEFAULT_LOCATION.to_string());
        match Config::retrieve() {
            Some(conf) => conf,
            None => {
                let conf = Config {
                    client_id: 0,
                    client_secret: "".to_string(),
                    domain: "".to_string(),
                    github_key: "".to_string(),
                };
                conf.save();
                println!("Created a new config.yml to {}", &location);
                conf
            }
        }
    }

    pub fn save(&self) {
        let serialized = serde_yaml::to_string(&self).expect("Failed to serialize");
        let location = env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
        match File::create(&location) {
            Ok(mut file) => file
                .write_all(serialized.as_bytes())
                .expect("Failed to write"),
            Err(e) => panic!("Failed to save config at {}\n{}", &location, e),
        }
    }

    fn retrieve() -> Option<Config> {
        let location = env::var(ENV_VAR).unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
        match File::open(&location) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_err(){
                    return None;
                };

                match serde_yaml::from_str(&contents) {
                    Ok(des) => Some(des),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}
