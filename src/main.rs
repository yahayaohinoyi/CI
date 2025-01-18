use CI::parse;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <ci_tool <YAML_FILE>>");
        std::process::exit(1);
    }
    let yaml_file_path = &args[1];
    let config = parse(yaml_file_path);
    
    println!("Hello, world!");
}