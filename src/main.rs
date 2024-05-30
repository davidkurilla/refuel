use diesel::{pg::PgConnection, Connection};
use diesel_migrations::{FileBasedMigrations, HarnessWithOutput, MigrationHarness};
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

    let table = "master_database";

    let username = toml_data
        .get(table)
        .and_then(|master_database| master_database.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let password = toml_data
        .get(table)
        .and_then(|master_database| master_database.get("password"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    
    let dbname = toml_data
        .get(table)
        .and_then(|master_database| master_database.get("dbname"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let db_url = format!("postgres://{}:{}@localhost:5432/{}",
        username, password, dbname
    );

    let mut conn = PgConnection::establish(&db_url)
    .expect("Unable to connect");

    let migrations = FileBasedMigrations::find_migrations_directory()
        .expect("Could not read migrations directory");

    let mut harness = HarnessWithOutput::write_to_stdout(&mut conn);

    harness.run_pending_migrations(migrations).expect("Couldn't run migrations");

    println!("Successfully ran migrations")
}

