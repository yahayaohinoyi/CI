use anyhow::Context;
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    name: String,
    triggers: Vec<Trigger>,
    jobs: HashMap<String, Job>,
}

impl Config {
    pub fn new(&self, name: String, triggers: Vec<Trigger>, jobs: HashMap<String, Job>) -> Self {
        Self {
            name,
            triggers,
            jobs,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Trigger {
    Push { branches: Vec<String> },
    PullRequest { paths: Vec<String> },
}

#[derive(Debug, Clone, Deserialize)]
pub enum Job {
    Build(JobMetadata),
    Test(JobMetadata),
    Deploy(JobMetadata),
    Lint(JobMetadata),
}

#[derive(Debug, Clone, Deserialize)]
pub enum Os {
    Linux(String),
    Mac(String),
    Windows(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobMetadata {
    runs_on: Os,
    steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    name: String,
    uses: Option<String>,
    run: Option<String>,
    with: Option<String>,
}

pub fn parse(yaml_file_path: &str) -> anyhow::Result<Config> {
    let yaml_config = std::fs::read_to_string(yaml_file_path)
        .with_context(|| format!("Cannot read file: {}", yaml_file_path))?;

    let config: Config = serde_yaml::from_str(&yaml_config)
        .with_context(|| format!("Failed to parse YAML content in file: {}", yaml_file_path))?;

    Ok(config)
}