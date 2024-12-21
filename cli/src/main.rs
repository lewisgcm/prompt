mod commands;

use clap::{arg, ArgAction, Command};
use std::error::Error;

fn cli() -> Command {
    let home_dir_arg = arg!(<DIR> "Directory of prompt configuration and chats. Defaults to <user home directory>.prompt")
        .long("prompt-dir")
        .short('D')
        .required(false);

    Command::new("prompt")
        .about("CLI to interact with AI agents")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("converse")
                .about("Converse with an AI agent")
                .arg(home_dir_arg.clone())
                .arg(arg!(<MODEL> "Model to use for the prompt").short('m').long("model"))
                .arg(
                    arg!(<CHAT_ID> "Chat identifier to store message history (letters, numbers and '_' only)")
                        .short('c')
                        .long("chat-id"),
                )
                .arg(
                    arg!(<INPUT_TEXT> "Input text to send the model"),
                )
                .arg(
                    arg!(<INPUT_IMAGE> "Input image to send the model")
                        .long("i"),
                )
                .arg(
                    arg!(<INPUT_DOCUMENT> "Input document to send the model")
                        .long("d"),
                ),
        )
        .subcommand(
            Command::new("test")
                .about("Test a bundled js tool or model plugin")
                .arg(arg!(<PATH> "Path of the bundled.js plugin").required(true)),
        )
        .subcommand(
            Command::new("setup")
                .about("Interactively configure prompt models, chats, or plugins")
                .arg(home_dir_arg.clone())
                .arg(arg!(<LIST_MODEL_PLUGINS> "List installed model plugins")
                    .long("list-model-plugins")
                    .action(ArgAction::SetTrue)
                    .required(false))
                .arg(arg!(<LIST_TOOL_PLUGINS> "List installed tool plugins")
                    .long("list-tool-plugins")
                    .action(ArgAction::SetTrue)
                    .required(false)),
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
        Some(("setup", sub_matches)) => {
            commands::setup::run_command(sub_matches).await?;
        }
        _ => println!("Command not found"),
    }

    Ok(())
}
