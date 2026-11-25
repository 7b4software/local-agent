use core::fmt;
use std::{path::Path, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model(String);
impl FromStr for Model {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err("model can not be empty".to_owned());
        }
        if !input
            .chars()
            .all(|c| c.is_ascii_lowercase() || "-/.:".contains(c))
        {
            return Err("Invalid model name".to_owned());
        }
        Ok(Self(input.to_owned()))
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Model {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Model to use
    pub model: Model,
    /// preamble
    pub preamble: Option<String>,
    /// Max turns
    #[serde(default = "default_max_turns")]
    pub max_turns: usize,
    #[serde(default = "default_tokens")]
    pub max_tokens: u64,
}

fn default_max_turns() -> usize {
    16
}

fn default_tokens() -> u64 {
    2048
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|error| format!("Could not read {path:?} cause: {error}"))?;
        toml::from_str(&content).map_err(|error| format!("Configuration fail {path:?} {error}"))
    }
}
