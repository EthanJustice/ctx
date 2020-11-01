// std
use std::collections::HashMap;
use std::fs::{create_dir, read_to_string, write};
use std::path::PathBuf;

// external
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

// local

fn scaffold() -> std::io::Result<()> {
    let bd = BaseDirs::new().expect("Failed to read configuration");
    let config_directory = bd.config_dir().join("ctx/");
    create_dir(config_directory.clone())?;
    Ok(write(
        config_directory.join("config.json"),
        "{ \"workspaces\": {} }",
    )?)
}

#[derive(Serialize, Deserialize)]
/// ctx's configuration, as read from the user's configuration file located in their configuration directory
pub struct Config {
    /// a list of all workspaces and the file system paths to them
    pub workspaces: HashMap<String, PathBuf>,
}

impl Config {
    pub fn get() -> Config {
        let bd = BaseDirs::new().expect("Failed to find config directory.");
        let config_directory = bd.config_dir().join("ctx");
        if config_directory.is_dir() == false {
            scaffold().expect("Failed to scaffold configuration.");
        }
        let config: Config = from_str(
            read_to_string(config_directory.join("config.json"))
                .expect("Failed to read config file!")
                .as_str(),
        )
        .expect("Failed to serialise configuration.");

        config
    }

    /// adds a workspace to the current configuration instance, which is then saved to the configuration file
    pub fn add_workspace<T: Into<String>>(
        &mut self,
        workspace: T,
        path: PathBuf,
    ) -> std::io::Result<()> {
        self.workspaces.insert(workspace.into(), path);
        self.save()
    }

    /// saves the deserialised self instance into the configuration file
    pub fn save(&self) -> std::io::Result<()> {
        let bd = BaseDirs::new().expect("Failed to find config directory.");
        let config_directory = bd.config_dir().join("ctx");
        write(
            config_directory.join("config.json"),
            to_string_pretty(self).expect("Failed to save configuration"),
        )
    }
}

pub struct Project {}
