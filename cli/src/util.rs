use anyhow::format_err;
use clap::ArgMatches;
use homedir::my_home;
use prompt_core::config::PROMPT_DEFAULT_DIRECTORY;
use std::path::PathBuf;

pub fn get_home_dir(sub_matches: &ArgMatches) -> Result<PathBuf, anyhow::Error> {
    let user_home_directory = my_home()?.map(|home_dir| {
        PathBuf::from(home_dir.to_str().unwrap().to_string()).join(PROMPT_DEFAULT_DIRECTORY)
    });

    Ok(sub_matches
        .get_one::<String>("DIR")
        .map(PathBuf::from)
        .or_else(|| user_home_directory)
        .ok_or_else(|| format_err!("could not resolve home directory"))?)
}
