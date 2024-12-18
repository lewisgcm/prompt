mod commands;
mod util;

use clap::{arg, Command};
use std::error::Error;

fn cli() -> Command {
    Command::new("prompt")
        .about("CLI to interact with generative AI agents")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("converse")
                .about("Converse with an LLM")
                .arg(
                    arg!(<CONFIG> "YAML configuration file for prompt")
                        .long("config")
                        .required(false)
                        .default_value("Yml"),
                )
                .arg(
                    arg!(<HOMEDIR> "directory used to store chat history")
                        .long("home-dir")
                        .required(false)
                        .default_value("."),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("test")
                .about("Test a Javascript prompt model or tool plugin")
                .arg(arg!(<PATH> "Path of the bundled.js plugin").required(true)),
        )
}

// Based on
// https://github.com/clap-rs/clap/blob/master/examples/git.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("test", sub_matches)) => {
            commands::test::run_command(sub_matches).await?;
        }
        _ => println!("Command not found"),
    }

    Ok(())
}
