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
    let toml_contents = match std::fs::read_to_string(toml_file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading TOML file: {}", e);
            return;
        }
    };
    let toml_data: Value = match toml_contents.parse() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing TOML file: {}", e);
            return;
        }
    };

    // Invoke get_key(String key)
    let username = toml_data["username"].as_str().unwrap_or_default(); //NO Expects
    let password = toml_data["password"].as_str().unwrap_or_default(); //No Expects
    let dbname = toml_data["dbname"].as_str().unwrap_or_default(); // No Expect

    let command = format!(
        "migration --database-url postgres://{}:{}@localhost:5432/{} run",
        username, password, dbname
    );

    let output = match Command::new("diesel").arg(&command).output() {
            Ok(out) => {
                println!("Command executed successfully");
                out
            },
            Err(e) => {
                eprintln!("Error executing command: {}", e);
                println!("Tip: Verify that 'diesel' is installed");
                return;
            }
        };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
