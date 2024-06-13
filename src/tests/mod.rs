#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use toml::Value;

    use crate::input_file::InputData;
    use crate::get_toml_table;

    #[test]
    fn test_input_file() {
        let toml_str =
         r#"username = "db_user"
            password = "db_pass"
            dbname = "db_name"
            host = "localhost"
            port = 5432"#;

        let toml_value = Value::from_str(toml_str).unwrap();

        let toml_table = InputData::read(&toml_value);

        let db_url = toml_table.postgres_url();
        assert_eq!("postgres://db_user:db_pass@localhost:5432/db_name", db_url);
    }

    #[test]
    fn test_given_toml(){
        let toml_str_table =
           "[database]\n\
            username = \"db_user\"\n\
            password = \"db_pass\"\n\
            dbname = \"db_name\"\n\
            host = \"localhost\"\n\
            port = 5432";
        
        let table_name = "database";
        let toml_value = Value::from_str(toml_str_table).unwrap();
        let table = get_toml_table(&table_name, &toml_value);
    
        assert!(table.is_table());
        
        let table_name = "";
        let table = get_toml_table(&table_name, &toml_value);
        assert!(table.is_table()); 

    }

}