use std::process::Command;
use toml::Value;
use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    toml_file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let toml_file = args.toml_file;
    let toml_contents = std::fs::read_to_string(toml_file).expect("Failed to read TOML file");
    let toml_data: Value = toml_contents.parse().expect("Failed to parse TOML data");

    let username = toml_data["username"].as_str().unwrap_or_default();
    let password = toml_data["password"].as_str().unwrap_or_default();
    let dbname = toml_data["dbname"].as_str().unwrap_or_default();

    let command = format!(
        "diesel migration --database-url postgres://{}:{}@localhost:5432/{} run",
        username, password, dbname
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
