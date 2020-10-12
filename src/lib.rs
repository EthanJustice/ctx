// std

// external
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    workspaces: Vec<String>,
}

impl Config {
    pub fn get() -> Config {
        Config {
            workspaces: Vec::new(),
        }
    }
}
