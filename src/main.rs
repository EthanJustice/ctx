// std
use std::path::PathBuf;

// external
use clap::{App, Arg, ArgMatches, SubCommand};
use open::that;

// local
use ctx::Config;

fn main() {
    let mut config = Config::get();
    let app = App::new("ctx")
        .about(env!("CARGO_PKG_DESCRIPTION")) // these are set at compile-time
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new workspace.")
                .arg(
                    Arg::with_name("INPUT")
                        .help("The directory to set as the new workspace")
                        .default_value("."),
                )
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .takes_value(true)
                        .help("The name to give the new workspace.")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("config").about("View and edit your config."))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add an item to a workspace")
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .help("The type of item to add")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("value")
                        .short("v")
                        .help("The value of the item being added")
                        .possible_values(&["link"])
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("workspace")
                        .short("w")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Change or remove items in the current workspace")
                .arg(Arg::with_name("d").help("Delete the specified item")),
        )
        .subcommand(
            SubCommand::with_name("launch")
                .about("Launch a workspace.")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .help("The name of the workspace to launch")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(subcommand) = app.subcommand_matches("new") {
        let workspace_dir_name = subcommand
            .value_of("INPUT")
            .expect("No directory provided, aborting...");
        let workspace_name = subcommand
            .value_of("name")
            .unwrap_or_else(|| workspace_dir_name);
        config
            .add_workspace(
                workspace_name,
                PathBuf::from(workspace_dir_name)
                    .canonicalize()
                    .expect("Failed to convert provided path to an absolute one."),
            )
            .expect("Failed to save new config.");
    } else if let Some(_subcommand) = app.subcommand_matches("config") {
    } else if let Some(subcommand) = app.subcommand_matches("add") {
        add(subcommand, config);
    } else if let Some(_subcommand) = app.subcommand_matches("edit") {
    } else if let Some(subcommand) = app.subcommand_matches("launch") {
        let workspace = config
            .workspaces
            .get(
                subcommand
                    .value_of("name")
                    .expect("No workspace name provided, aborting..."),
            )
            .expect("Couldn't find a workspace with that name.");
        workspace.links.iter().for_each(|i| {
            that(i).expect("Failed to open link.");
        });
    }
}

fn add(subcommand: &ArgMatches, mut config: Config) {
    let workspace_to_edit = subcommand
        .value_of("workspace")
        .expect("Please provide a workspace name.");
    let workspace_items = config
        .workspaces
        .get_mut(workspace_to_edit)
        .expect("No workspace found by that name, aborting...");

    let type_to_add = subcommand
        .value_of("type")
        .expect("No type found, aborting...");
    let value = subcommand
        .value_of("value")
        .expect("No new value provided, aborting...");

    if type_to_add == "link" {
        workspace_items.links.push(value.into());
    }

    println!(
        "Added item {} of type {} to workspace {} successfully.",
        value, type_to_add, workspace_to_edit
    );
}
