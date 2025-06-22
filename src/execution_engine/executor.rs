use crate::Job;
use anyhow::Context;
use bollard::container::CreateContainerOptions;
use bollard::models::ContainerCreateBody;
use bollard::Docker;
use uuid::Uuid;

pub async fn execute(job: &Job) -> anyhow::Result<String> {
    let docker = Docker::connect_with_local_defaults().context("Failed to connect to docker")?;

    let id = Uuid::new_v4();
    let options = CreateContainerOptions {
        name: format!("docker-container-{}", id),
        platform: Some("ubuntu".to_string()),
    };

    let cmd = make_command_for_job(job);

    let config = ContainerCreateBody {
        hostname: Some("localhost".to_owned()),
        cmd: Some(cmd),
        ..Default::default()
    };

    let container = docker
        .create_container(Some(options), config)
        .await
        .context("Failed to create docker container")?;

    Ok(container.id)
}

fn make_command_for_job(_: &Job) -> Vec<String> {
    todo!();
}
