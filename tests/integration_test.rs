use std::{fmt::format, process::Command};
use testcontainers::{core::WaitFor, runners::SyncRunner, GenericImage};
use toml::Value;

#[test]
fn postgres_migrations_test() {
    let test_toml_path = "test.toml";
    let test_toml_str = std::fs::read_to_string(test_toml_path).expect("Unable to read toml file");
    let test_toml_data: Value = test_toml_str.parse().expect("Unable to parse toml file");

    let pq_user = test_toml_data.get("username").unwrap().as_str().unwrap();
    let pq_pass = test_toml_data.get("password").unwrap().as_str().unwrap();
    let pq_dbname = test_toml_data.get("dbname").unwrap().as_str().unwrap();
    let pq_port = test_toml_data.get("port").unwrap().as_integer().unwrap() as u16;

    let postgres_container = GenericImage::new("postgres", "latest")
        //.with_exposed_port(pq_port)
        .with_env_var("POSTGRES_USER", pq_user)
        .with_env_var("POSTGRES_PASSWORD", pq_pass)
        .with_env_var("POSTGRES_DB", pq_dbname)
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .expect("Postgres started");

    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html
    let binary_path = env!("CARGO_BIN_EXE_refuel");
    
    let mut binding = Command::new(binary_path);
    let cmd = binding.args(["--toml-file", test_toml_path]);
    
    let output = cmd.output();

    assert!(output.is_ok());

    postgres_container.stop().expect("Postgres stopped");
}