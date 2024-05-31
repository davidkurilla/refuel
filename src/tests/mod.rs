#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use toml::Value;

    use crate::input_file::InputData;

    #[test]
    fn test_input_file() {
        let toml_str =
           "username = \"db_user\"\n\
            password = \"db_pass\"\n\
            dbname = \"db_name\"\n\
            host = \"localhost\"\n\
            port = 5432";

        let toml_value = Value::from_str(toml_str).unwrap();

        let toml_table = InputData::read(&toml_value);

        let db_url = toml_table.postgres_url();
        assert_eq!("postgres://db_user:db_pass@localhost:5432/db_name", db_url);
    }
}