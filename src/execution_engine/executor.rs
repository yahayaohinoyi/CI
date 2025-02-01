// acquires somne docker resource 
// maybe i should have some file called provider that houses this and is responsible for all things docker resource interactions 

// run command in those resources, get logs and output

use tokio::process::Command;


pub async fn acquire_resource() -> anyhow::Result<()> {
    let container_process = Command::new("docker");
    Ok(())
}