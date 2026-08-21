use anyhow::Context;
use bollard::container::LogOutput;
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{
    CreateContainerOptionsBuilder, CreateImageOptionsBuilder, RemoveContainerOptionsBuilder,
};
use bollard::Docker;
use futures_util::TryStreamExt;
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
        let container = self
            .start_container()
            .await
            .context("Cannot start container")?;

        let result = self.execute_in_container(&container.id).await;
        let cleanup = self.remove_container(&container.id).await;

        result?;
        cleanup?;

        Ok(String::from("container executed"))
    }

    async fn execute_in_container(&self, container_id: &str) -> anyhow::Result<()> {
        let create_exec_option = CreateExecOptions {
            cmd: Some(vec!["ps", "-ef"]),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let exec = self
            .docker
            .create_exec(container_id, create_exec_option)
            .await
            .context("Failed to create exec")?;

        let start_exec_options = StartExecOptions {
            detach: false,
            tty: false,
            output_capacity: Some(8 * 1024),
        };

        let result = self
            .docker
            .start_exec(&exec.id, Some(start_exec_options))
            .await
            .context("Failed to start exec")?;

        match result {
            StartExecResults::Attached { output, .. } => {
                let logs = output.try_collect::<Vec<LogOutput>>().await?;
                for log in logs {
                    print!("{}", String::from_utf8_lossy(&log.into_bytes()));
                }
                Ok(())
            }
            StartExecResults::Detached => anyhow::bail!("Expected an attached exec session"),
        }
    }

    async fn create_image(&self) -> anyhow::Result<()> {
        let options = CreateImageOptionsBuilder::default()
            .from_image("alpine")
            .tag("3.20")
            .build();

        self.docker
            .create_image(Some(options), None, None)
            .try_collect::<Vec<_>>()
            .await?;

        Ok(())
    }

    async fn create_container(&self) -> anyhow::Result<bollard::config::ContainerCreateResponse> {
        let docker = &self.docker;
        self.create_image().await?;

        let options = CreateContainerOptionsBuilder::default()
            .name(&format!("docker-container-{}", Uuid::new_v4()))
            .build();

        let cmd = Some(vec![
            "sh".to_owned(),
            "-c".to_owned(),
            "while :; do sleep 3600; done".to_owned(),
        ]);

        let config = ContainerCreateBody {
            image: Some("alpine:3.20".to_owned()),
            hostname: Some("localhost".to_owned()),
            cmd,
            ..Default::default()
        };

        let container = docker
            .create_container(Some(options), config)
            .await
            .context("Failed to create docker container")?;

        Ok(container)
    }

    async fn start_container(&self) -> anyhow::Result<bollard::config::ContainerCreateResponse> {
        let container = self
            .create_container()
            .await
            .context("Failed to create docker container")?;

        self.docker
            .start_container(&container.id, None)
            .await
            .context("Failed to start container")?;

        Ok(container)
    }

    async fn remove_container(&self, container_id: &str) -> anyhow::Result<()> {
        let options = RemoveContainerOptionsBuilder::default().force(true).build();

        self.docker
            .remove_container(container_id, Some(options))
            .await
            .context("Failed to remove container")
    }
}
