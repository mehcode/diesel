use std::borrow::Cow;
use std::env;
use std::error::Error;

pub fn extract_database_url<'a>(url: &'a str) -> Result<Cow<'a, str>, String> {
    if url.starts_with("dotenv:") {
        try!(load_dotenv_file());
        return extract_database_url(&url[3..]);
    } else if url.starts_with("env:") {
        let var_name = &url[4..];
        env::var(var_name)
            .map(Cow::Owned)
            .map_err(|e| {
                format!("Failed to load environment variable {}: {}",
                    var_name, e.description())
            })
    } else if url.starts_with("config:") {
        get_database_url_from_config(&url[6..])
    } else {
        Ok(Cow::Borrowed(url))
    }
}

#[cfg(feature = "dotenv")]
fn load_dotenv_file() -> Result<(), String> {
    use dotenv::dotenv;

    dotenv().ok();
    Ok(())
}

#[cfg(not(feature = "dotenv"))]
fn load_dotenv_file() -> Result<(), String> {
    Err(String::from("The dotenv feature is required to use strings starting \
        with `dotenv:`"))
}

#[cfg(feature = "config")]
fn get_database_url_from_config<'a, 'b>(key: &'a str) -> Result<Cow<'b, str>, String> {
    use config;

    config::get_str(key).ok_or_else(|| format!("configuration property {} is not defined", key))
}

#[cfg(not(feature = "config"))]
fn get_database_url_from_config() -> Result<(), String> {
    Err(String::from("The config feature is required to use strings starting \
        with `config:`"))
}

#[test]
fn extract_database_url_returns_the_given_string() {
    assert_eq!("foo", extract_database_url("foo").unwrap());
    assert_eq!("bar", extract_database_url("bar").unwrap());
}

#[test]
fn extract_database_url_returns_env_vars() {
    env::set_var("lolvar", "lololol");
    env::set_var("trolvar", "trolololol");
    assert_eq!("lololol", extract_database_url("env:lolvar").unwrap());
    assert_eq!("trolololol", extract_database_url("env:trolvar").unwrap());
}

#[test]
fn extract_database_url_errors_if_env_var_is_unset() {
    env::remove_var("selfdestructvar");
    assert!(extract_database_url("env:selfdestructvar").is_err());
}
