use CI::{execution_engine, parse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <ci_tool <YAML_FILE>>");
        std::process::exit(1);
    }
    let yaml_file_path = &args[1];
    let config = parse(yaml_file_path)?;
    eprintln!("config file content: {}", config);

    let executor = execution_engine::executor::Executor::new()?;
    let msg = executor.execute().await?;
    eprintln!("running executor: {}", msg);

    Ok(())
}
