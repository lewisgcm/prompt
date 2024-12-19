use inquire::validator::Validation;
use regex::Regex;
use std::error::Error;
use std::fs;

pub fn plugin_location_validator<'a>(
    input: &'a str,
) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let file_exists = fs::exists(input)?;
    match file_exists {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            format!("Plugin file does not exist: {}", input).into(),
        )),
    }
}

pub fn plugin_name_validator<'a>(
    input: &'a str,
) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let regex = Regex::new("[0-9_A-Za-z]+")?;
    match regex.is_match(input) {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            "Plugin names can only contain letters, numbers or '_'".into(),
        )),
    }
}
