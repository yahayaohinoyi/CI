use crate::Step;
use anyhow::{Context, Ok};
use bollard::container::{CreateContainerOptions, StartContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecOptions};
use bollard::models::ContainerCreateBody;
use bollard::Docker;
use uuid::Uuid;

#[derive(Clone)]
pub struct Executor {
    docker: Docker,
}

impl Executor {
    pub fn new() -> anyhow::Result<Self> {
        let docker =
            Docker::connect_with_defaults().context("Failed to connect to Docker daemon")?;
        Ok(Self { docker })
    }

    pub async fn execute(&self) -> anyhow::Result<String> {
        let _ = self
            .start_container()
            .await
            .context("Cannot start container");
        let exec_id = Uuid::new_v4();
        let create_exec_option = CreateExecOptions {
            cmd: None,
            ..Default::default()
        };

        self.docker
            .create_exec(&format!("{}", exec_id), create_exec_option)
            .await;

        let start_exec_options = StartExecOptions {
            detach: true,
            tty: true,
            output_capacity: Some(8 * 1024),
        };

        self.docker
            .start_exec(&format!("{}", exec_id), Some(start_exec_options));

        Ok(String::from("container executing"))
    }

    async fn start_container(&self) -> anyhow::Result<()> {
        let container = self
            .create_container()
            .await
            .inspect_err(|err| eprintln!("Error encountered while creating container"))?;

        let start_container_options = StartContainerOptions::<String> { detach_keys: None };

        let result = self
            .docker
            .start_container(&container, None::<StartContainerOptions<String>>)
            .await?;

        Ok(())
    }

    async fn create_container(&self) -> anyhow::Result<String> {
        let docker = &self.docker;

        let id = Uuid::new_v4();
        let options = CreateContainerOptions {
            name: format!("docker-container-{}", id),
            platform: Some("linux/amd64".to_string()),
        };

        let cmd = Some(vec![String::from("echo"), String::from("Hello World!")]);

        let config = ContainerCreateBody {
            hostname: Some("localhost".to_owned()),
            cmd,
            ..Default::default()
        };

        let container = docker
            .create_container(Some(options), config)
            .await
            .context("Failed to create docker container")?;

        Ok(container.id)
    }

    fn make_command_for_step(step: &Step) -> Option<Vec<String>> {
        let command = step
            .run
            .as_ref()
            .map(|cmd| cmd.split_whitespace())
            .into_iter()
            .flatten()
            .map(str::to_string)
            .collect::<Vec<String>>();

        Some(command)
    }
}
