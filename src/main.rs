// std

// external
use clap::{App, Arg, SubCommand};

// local
use ctx::Config;

fn main() {
    let config = Config::get();
    let app = App::new("ctx")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("new").about("Create a new workspace."))
        .subcommand(SubCommand::with_name("config").about("View and edit your config."))
        .subcommand(SubCommand::with_name("add").about("Add an item to the current workspace"))
        .subcommand(
            SubCommand::with_name("edit")
                .about("Change or remove items in the current workspace")
                .arg(Arg::with_name("d").help("Remove the specified item")),
        )
        .get_matches();

    if let Some(_subcommand) = app.subcommand_matches("new") {
        // ...
    } else if let Some(_subcommand) = app.subcommand_matches("config") {
    } else if let Some(_subcommand) = app.subcommand_matches("add") {
    } else if let Some(_subcommand) = app.subcommand_matches("edit") {
    }
}
