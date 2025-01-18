use anyhow::Context;
use serde::Deserialize;
use serde::Deserializer;
use serde;
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    name: String,
    triggers: Vec<Trigger>,
    jobs: HashMap<String, Job>,
}

impl Config {
    pub fn new(name: String, triggers: Vec<Trigger>, jobs: HashMap<String, Job>) -> Self {
        Self {
            name,
            triggers,
            jobs,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, triggers {:?}, job {:?}",
            self.name, self.triggers, self.jobs
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Trigger {
    Push { push: PushTrigger },
    PullRequest { pull_request: PullRequestTrigger },
}

#[derive(Debug, Clone, Deserialize)]
pub struct PushTrigger {
    branches: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestTrigger {
    paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Os {
    Linux(String),
    Mac(String),
    Windows(String),
}

impl<'de> Deserialize<'de> for Os {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let os_str: String = Deserialize::deserialize(deserializer)?;

        match os_str.as_str() {
            "ubuntu-latest" => Ok(Os::Linux(os_str)),
            "linux" => Ok(Os::Linux(os_str)),
            "mac" => Ok(Os::Mac(os_str)),
            "windows" => Ok(Os::Windows(os_str)),
            _ => Err(serde::de::Error::unknown_variant(
                &os_str,
                &["ubuntu-latest", "linux", "mac", "windows"],
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    runs_on: Os,
    steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    name: String,
    uses: Option<String>,
    run: Option<String>,
    #[serde(default)]
    with: Option<HashMap<String, String>>,
}

pub fn parse(yaml_file_path: &str) -> anyhow::Result<Config> {
    let yaml_config = std::fs::read_to_string(yaml_file_path)
        .with_context(|| format!("Cannot read file: {}", yaml_file_path))?;

    let config: Config = serde_yaml::from_str(&yaml_config)
        .with_context(|| format!("Failed to parse YAML content in file: {}", yaml_file_path))?;

    Ok(config)
}
