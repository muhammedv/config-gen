extern crate serde_yaml;

use clap::App as ClapApp;
use clap::Arg;
use options::get_host;
use options::ConfigError;
use options::ProxyOpts;
use serde_yaml::Value;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct PresetConfig {
    pub name: String,
    pub options: Value,
}

#[derive(Deserialize, Debug)]
pub struct ProgramConfig {
    pub presets: Vec<PresetConfig>,
}

pub fn get_config_contents_from_file(maybe_path: &str) -> Result<String, ProgramStartError> {
    let mut file = File::open(maybe_path).map_err(|_| ProgramStartError::ConfigFileOpen)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| ProgramStartError::ConfigFileRead)?;
    Ok(contents)
}

pub fn get_program_config_from_string(input: &str) -> Result<ProgramConfig, ProgramStartError> {
    serde_yaml::from_str(input).map_err(|e| ProgramStartError::ConfigParseError(e))
}

#[test]
fn test_get_program_config_from_string() {
    let i = r#"
presets:
  - name: m2
    options:
      require_path: /js/require.js
      bundle_config: file:test/fixtures/bundle-config.yaml
      auth_basic:
        username: shane
        password: other
    "#;
    let o = get_program_config_from_string(i);
    println!("{:#?}", o);
}

#[derive(Debug)]
pub enum ProgramStartError {
    ConfigFileOpen,
    ConfigFileRead,
    ConfigParseError(serde_yaml::Error),
    ConfigCliError(ConfigError),
}

impl std::fmt::Display for ProgramStartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProgramStartError::ConfigParseError(e) => write!(f, "could not parse config, {}", e),
            ProgramStartError::ConfigCliError(_e) => {
                write!(f, "could not parse incoming options from CLI")
            }
            ProgramStartError::ConfigFileOpen => {
                write!(f, "config file not found")
            }
            ProgramStartError::ConfigFileRead => {
                write!(f, "config file content could not be read")
            }
        }
    }
}

///
/// Options that come in via the CLI flags
///
pub fn get_program_config_from_cli() -> Result<ProxyOpts, ProgramStartError> {
    let matches = ClapApp::new("bs-rust")
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true),
        ).get_matches();

    match get_host(matches.value_of("input").unwrap_or("")) {
        Ok((host, scheme)) => Ok(ProxyOpts::new(host, scheme)
            .with_port(matches.value_of("port").unwrap_or("8080").parse().unwrap())),
        Err(err) => Err(ProgramStartError::ConfigCliError(err)),
    }
}
