// std
use std::collections::HashMap;
use std::fs::{create_dir, read_to_string, write};
use std::path::PathBuf;

// external
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

// local

/// generates configuration file and folder used by ctx in the user's configuration directory
fn scaffold() -> std::io::Result<()> {
    let config_directory = get_config_dir();
    if config_directory.is_dir() == false {
        create_dir(config_directory.clone())?;
    }
    Ok(write(
        config_directory.join("config.json"),
        "{ \"workspaces\": {}, \"workspace_paths\": {} }",
    )?)
}

/// returns ctx's directory within the user's configuration directory
pub fn get_config_dir() -> PathBuf {
    let bd = BaseDirs::new().expect("Failed to find configuration directory.");
    bd.config_dir().join("ctx/")
}

#[derive(Serialize, Debug, Clone, Deserialize)]
/// ctx's configuration, as read from the user's configuration file located in their configuration directory
pub struct Config {
    /// a list of all workspaces and the file system paths to them
    pub workspace_paths: HashMap<String, PathBuf>,
    /// a list of workspaces and their corresponding items (Workspace struct)
    pub workspaces: HashMap<String, Workspace>,
}

impl Config {
    pub fn get() -> Config {
        let config_directory = get_config_dir();
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
        self.workspace_paths.insert(workspace.into(), path);
        self.save()
    }

    /// saves the deserialised self instance into the configuration file
    pub fn save(&self) -> std::io::Result<()> {
        let config_directory = get_config_dir();
        write(
            config_directory.join("config.json"),
            to_string_pretty(self).expect("Failed to save configuration"),
        )
    }
}

/// represents a workspace
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Workspace {
    pub links: Vec<String>,
    pub tasks: HashMap<String, bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    // tests serialisation of workspace items
    #[test]
    pub fn new_workspace() {
        let mut hm = HashMap::new();
        hm.insert(String::from("This is a task, that isn't completed."), false);
        let workspace = Workspace {
            links: vec![String::from("https://rust-lang.org")],
            tasks: hm,
        };
        assert_eq!(
            workspace.links.get(0).unwrap().clone(),
            String::from("https://rust-lang.org")
        );
    }

    // tests to ensure configuration file is saved in the proper place
    #[test]
    pub fn config_path() {
        let config = get_config_dir();
        // for testing on local machines, wipe in case it exists already
        if config.is_dir() == true {
            remove_dir_all(config.clone()).unwrap();
        }

        assert_eq!(config.is_dir(), false);
        Config::get();
        assert_eq!(config.is_dir(), true);

        // cleanup
        remove_dir_all(config.clone()).unwrap();
    }

    #[test]
    pub fn config_init() {
        // remove directory beforehand, if it exists
        let config_dir = get_config_dir();
        if config_dir.is_dir() == true {
            remove_dir_all(get_config_dir()).unwrap();
        }

        let config = Config::get();
        assert_eq!(config.workspace_paths.len(), 0);
        assert_eq!(config.workspaces.len(), 0);
        assert_eq!(
            read_to_string(config_dir.join("config.json")).unwrap(),
            String::from("{ \"workspaces\": {}, \"workspace_paths\": {} }")
        );
    }
}
