// std

// external
use clap::{App, Arg, SubCommand};

// local

fn main() {
    let app = App::new("ctx")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("new").help_short("Create a new workspace."))
        .subcommand(SubCommand::with_name("config").help_short("View and edit your config."))
        .get_matches();

    if let Some(_subcommand) = app.subcommand_matches("new") {
        // ...
    }
}
