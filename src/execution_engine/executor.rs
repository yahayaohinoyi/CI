use crate::Job;
use crate::Step;
use anyhow::Context;
use bollard::container::CreateContainerOptions;
use bollard::models::ContainerCreateBody;
use bollard::Docker;
use uuid::Uuid;

pub async fn gen_container_logs(_: String) -> String {
    todo!()
}

pub async fn start_container(_: String) -> anyhow::Result<()> {
    todo!()
}

pub async fn create_container(step: &Step) -> anyhow::Result<String> {
    let docker = Docker::connect_with_local_defaults().context("Failed to connect to docker")?;

    let id = Uuid::new_v4();
    let options = CreateContainerOptions {
        name: format!("docker-container-{}", id),
        platform: Some("ubuntu".to_string()),
    };

    let cmd = make_command_for_job(step);

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

fn make_command_for_job(step: &Step) -> Option<Vec<String>> {
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
